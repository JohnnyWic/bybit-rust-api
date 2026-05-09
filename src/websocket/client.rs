//! WebSocket client implementation.

use futures_util::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::{RwLock, mpsc};
use tokio::time::{Duration, interval};
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async, tungstenite::Message};
use tracing::{debug, error, info, warn};

use crate::auth::{generate_ws_signature, get_timestamp};
use crate::config::WsConfig;
use crate::error::{BybitError, Result};
use crate::websocket::models::*;
use crate::{MAINNET_WS_TRADE, TESTNET_WS_TRADE};

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;
type Callback = Arc<dyn Fn(WsMessage) + Send + Sync>;

/// WebSocket client for Bybit streaming API.
pub struct BybitWebSocket {
    config: WsConfig,
    subscriptions: Arc<RwLock<Vec<String>>>,
    callbacks: Arc<RwLock<HashMap<String, Callback>>>,
    tx: Option<mpsc::Sender<Message>>,
    is_connected: Arc<RwLock<bool>>,
}

impl BybitWebSocket {
    /// Create a new public WebSocket client.
    pub fn public(url: &str) -> Self {
        Self {
            config: WsConfig::public(url),
            subscriptions: Arc::new(RwLock::new(Vec::new())),
            callbacks: Arc::new(RwLock::new(HashMap::new())),
            tx: None,
            is_connected: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new private WebSocket client.
    pub fn private(api_key: &str, api_secret: &str, url: &str) -> Self {
        Self {
            config: WsConfig::private(api_key, api_secret).with_url(url),
            subscriptions: Arc::new(RwLock::new(Vec::new())),
            callbacks: Arc::new(RwLock::new(HashMap::new())),
            tx: None,
            is_connected: Arc::new(RwLock::new(false)),
        }
    }

    /// Connect to the WebSocket server.
    pub async fn connect(&mut self) -> Result<()> {
        let url = &self.config.url;
        info!(url = %url, "Connecting to WebSocket");

        let (ws_stream, _) = connect_async(url)
            .await
            .map_err(|e| BybitError::WebSocket(Box::new(e)))?;

        let (write, read) = ws_stream.split();

        // Create channel for sending messages
        let (tx, mut rx) = mpsc::channel::<Message>(100);
        self.tx = Some(tx.clone());

        // Set connected flag
        *self.is_connected.write().await = true;

        // Spawn write task
        let write = Arc::new(tokio::sync::Mutex::new(write));
        let write_clone = write.clone();
        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                let mut w = write_clone.lock().await;
                if let Err(e) = w.send(msg).await {
                    error!("Failed to send message: {}", e);
                    break;
                }
            }
        });

        // Authenticate if private channel
        if self.config.api_key.is_some() {
            self.authenticate().await?;
        }

        // Spawn ping task
        let tx_ping = tx.clone();
        let ping_interval = self.config.ping_interval;
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(ping_interval));
            loop {
                interval.tick().await;
                let ping = WsPing::new();
                let msg = serde_json::to_string(&ping).unwrap_or_default();
                if tx_ping.send(Message::Text(msg)).await.is_err() {
                    break;
                }
                debug!("Ping sent");
            }
        });

        // Spawn read task
        let callbacks = self.callbacks.clone();
        let is_connected = self.is_connected.clone();
        let subscriptions = self.subscriptions.clone();
        let config = self.config.clone();
        let tx_reconnect = tx.clone();

        tokio::spawn(async move {
            Self::handle_messages(
                read,
                callbacks,
                is_connected,
                subscriptions,
                config,
                tx_reconnect,
            )
            .await;
        });

        info!("WebSocket connected");
        Ok(())
    }

    /// Handle incoming messages.
    async fn handle_messages(
        mut read: futures_util::stream::SplitStream<WsStream>,
        callbacks: Arc<RwLock<HashMap<String, Callback>>>,
        is_connected: Arc<RwLock<bool>>,
        _subscriptions: Arc<RwLock<Vec<String>>>,
        _config: WsConfig,
        _tx: mpsc::Sender<Message>,
    ) {
        while let Some(msg_result) = read.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    // Try to parse as JSON
                    let json: serde_json::Value = match serde_json::from_str(&text) {
                        Ok(v) => v,
                        Err(e) => {
                            warn!(
                                "Failed to parse message: {}, text: {}",
                                e,
                                &text[..text.len().min(200)]
                            );
                            continue; // Don't panic, continue processing
                        }
                    };

                    // Handle different message types
                    if is_pong(&json) {
                        debug!("Pong received");
                        continue;
                    }

                    if is_auth_response(&json) {
                        if json
                            .get("success")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                            || json.get("retCode").and_then(|v| v.as_i64()) == Some(0)
                        // ^^^ this is for *_WS_TRADE ^^^
                        // https://bybit-exchange.github.io/docs/v5/websocket/trade/guideline#response-parameters
                        {
                            info!("Authentication successful");
                        } else {
                            error!("Authentication failed: {:?}", json);
                        }
                        continue;
                    }

                    if is_subscription_response(&json) {
                        if json
                            .get("success")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false)
                        {
                            debug!("Subscription successful");
                        } else {
                            warn!("Subscription failed: {:?}", json);
                        }
                        continue;
                    }

                    // Handle data message
                    if is_data_message(&json) {
                        if let Ok(ws_msg) = serde_json::from_value::<WsMessage>(json) {
                            let cbs = callbacks.read().await;
                            if let Some(callback) = cbs.get(&ws_msg.topic) {
                                callback(ws_msg.clone());
                            } else {
                                // Try to find matching callback by prefix
                                for (topic, callback) in cbs.iter() {
                                    if ws_msg
                                        .topic
                                        .starts_with(topic.split('.').next().unwrap_or(""))
                                    {
                                        callback(ws_msg.clone());
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
                Ok(Message::Ping(_)) => {
                    debug!("Received ping frame");
                    // Tungstenite handles pong automatically
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket closed");
                    *is_connected.write().await = false;
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    *is_connected.write().await = false;
                    break;
                }
                _ => {}
            }
        }
    }

    /// Authenticate with the server (for private channels).
    async fn authenticate(&self) -> Result<()> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| BybitError::Auth("API key not set".into()))?;
        let api_secret = self
            .config
            .api_secret
            .as_ref()
            .ok_or_else(|| BybitError::Auth("API secret not set".into()))?;

        let expires = get_timestamp() + 10000;
        let signature = generate_ws_signature(api_secret, expires);

        // if self.config.url == TESTNET_WS_TRADE || self.config.url == MAINNET_WS_TRADE {
        //     let auth_msg = WsTradeAuthResponse
        // }
        let auth_msg = WsAuthRequest {
            req_id: uuid::Uuid::new_v4().to_string(),
            op: "auth".to_string(),
            args: vec![
                serde_json::Value::String(api_key.clone()),
                serde_json::Value::Number(expires.into()),
                serde_json::Value::String(signature),
            ],
        };

        let msg = serde_json::to_string(&auth_msg).map_err(|e| BybitError::Parse(e.to_string()))?;

        self.send(msg).await?;
        info!("Authentication request sent");
        Ok(())
    }

    /// Subscribe to topics.
    ///
    /// # Arguments
    /// * `topics` - List of topics to subscribe
    /// * `callback` - Callback function for received messages
    pub async fn subscribe<F>(&mut self, topics: Vec<String>, callback: F) -> Result<()>
    where
        F: Fn(WsMessage) + Send + Sync + 'static,
    {
        let callback = Arc::new(callback) as Callback;

        // Register callbacks
        {
            let mut cbs = self.callbacks.write().await;
            for topic in &topics {
                cbs.insert(topic.clone(), callback.clone());
            }
        }

        // Store subscriptions
        {
            let mut subs = self.subscriptions.write().await;
            subs.extend(topics.clone());
        }

        // Send subscription request
        let sub_msg = WsRequest {
            req_id: uuid::Uuid::new_v4().to_string(),
            op: "subscribe".to_string(),
            args: topics,
        };

        let msg = serde_json::to_string(&sub_msg).map_err(|e| BybitError::Parse(e.to_string()))?;

        self.send(msg).await
    }

    /// Unsubscribe from topics.
    pub async fn unsubscribe(&mut self, topics: Vec<String>) -> Result<()> {
        // Remove callbacks
        {
            let mut cbs = self.callbacks.write().await;
            for topic in &topics {
                cbs.remove(topic);
            }
        }

        // Remove from subscriptions
        {
            let mut subs = self.subscriptions.write().await;
            subs.retain(|t| !topics.contains(t));
        }

        // Send unsubscribe request
        let unsub_msg = WsRequest {
            req_id: uuid::Uuid::new_v4().to_string(),
            op: "unsubscribe".to_string(),
            args: topics,
        };

        let msg =
            serde_json::to_string(&unsub_msg).map_err(|e| BybitError::Parse(e.to_string()))?;

        self.send(msg).await
    }

    /// Send a message.
    async fn send(&self, msg: String) -> Result<()> {
        if let Some(tx) = &self.tx {
            tx.send(Message::Text(msg)).await.map_err(|_| {
                BybitError::WebSocket(Box::new(
                    tokio_tungstenite::tungstenite::Error::AlreadyClosed,
                ))
            })?;
        }
        Ok(())
    }

    /// Check if connected.
    pub async fn is_connected(&self) -> bool {
        *self.is_connected.read().await
    }

    /// Disconnect from the server.
    pub async fn disconnect(&mut self) -> Result<()> {
        *self.is_connected.write().await = false;
        self.tx = None;
        info!("WebSocket disconnected");
        Ok(())
    }
}
