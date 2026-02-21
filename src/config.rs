//! Configuration for the Bybit API client.

use std::time::Duration;

use crate::constants::{MAINNET, MAINNET_WS_PRIVATE, MAINNET_WS_PUBLIC_LINEAR};

/// Configuration for the Bybit HTTP client.
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// API key for authentication
    pub api_key: String,
    /// API secret for signing requests
    pub api_secret: String,
    /// Base URL for REST API
    pub base_url: String,
    /// Request timeout
    pub timeout: Duration,
    /// Receive window for timestamp validation (milliseconds)
    pub recv_window: u64,
    /// Enable debug logging
    pub debug: bool,
}

impl ClientConfig {
    /// Create a new configuration builder.
    pub fn builder(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
    ) -> ClientConfigBuilder {
        ClientConfigBuilder::new(api_key, api_secret)
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_secret: String::new(),
            base_url: MAINNET.to_string(),
            timeout: Duration::from_secs(30),
            recv_window: 5000,
            debug: false,
        }
    }
}

/// Builder for ClientConfig.
#[derive(Debug, Clone)]
pub struct ClientConfigBuilder {
    config: ClientConfig,
}

impl ClientConfigBuilder {
    /// Create a new builder with required credentials.
    pub fn new(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            config: ClientConfig {
                api_key: api_key.into(),
                api_secret: api_secret.into(),
                ..Default::default()
            },
        }
    }

    /// Set the base URL for REST API.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.config.base_url = url.into();
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Set the receive window (milliseconds).
    pub fn recv_window(mut self, recv_window: u64) -> Self {
        self.config.recv_window = recv_window;
        self
    }

    /// Enable or disable debug logging.
    pub fn debug(mut self, debug: bool) -> Self {
        self.config.debug = debug;
        self
    }

    /// Build the configuration.
    pub fn build(self) -> ClientConfig {
        self.config
    }
}

/// Configuration for WebSocket client.
#[derive(Debug, Clone)]
pub struct WsConfig {
    /// API key for private channels
    pub api_key: Option<String>,
    /// API secret for private channels
    pub api_secret: Option<String>,
    /// WebSocket URL
    pub url: String,
    /// Ping interval in seconds
    pub ping_interval: u64,
    /// Maximum reconnection attempts (0 = infinite)
    pub max_reconnect_attempts: u32,
    /// Reconnection delay in seconds
    pub reconnect_delay: u64,
}

impl Default for WsConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            url: MAINNET_WS_PUBLIC_LINEAR.to_string(),
            ping_interval: 20,
            max_reconnect_attempts: 10,
            reconnect_delay: 5,
        }
    }
}

impl WsConfig {
    /// Create a public WebSocket configuration.
    pub fn public(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Default::default()
        }
    }

    /// Create a private WebSocket configuration.
    pub fn private(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            api_secret: Some(api_secret.into()),
            url: MAINNET_WS_PRIVATE.to_string(),
            ..Default::default()
        }
    }

    /// Set the WebSocket URL.
    pub fn with_url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    /// Set the ping interval in seconds.
    pub fn with_ping_interval(mut self, interval: u64) -> Self {
        self.ping_interval = interval;
        self
    }

    /// Set the maximum reconnection attempts.
    pub fn with_max_reconnect_attempts(mut self, attempts: u32) -> Self {
        self.max_reconnect_attempts = attempts;
        self
    }
}
