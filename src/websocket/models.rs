//! WebSocket message models.

use serde::{Deserialize, Serialize};

/// WebSocket operation request.
#[derive(Debug, Clone, Serialize)]
pub struct WsRequest {
    /// Request ID
    pub req_id: String,
    /// Operation type
    pub op: String,
    /// Arguments
    pub args: Vec<String>,
}

/// WebSocket authentication request.
#[derive(Debug, Clone, Serialize)]
pub struct WsAuthRequest {
    /// Request ID
    pub req_id: String,
    /// Operation type
    pub op: String,
    /// Arguments [api_key, expires, signature]
    pub args: Vec<serde_json::Value>,
}

/// WebSocket response.
#[derive(Debug, Clone, Deserialize)]
pub struct WsResponse {
    /// Success flag
    #[serde(default)]
    pub success: Option<bool>,
    /// Return message
    #[serde(default)]
    pub ret_msg: Option<String>,
    /// Connection ID
    #[serde(default)]
    pub conn_id: Option<String>,
    /// Request ID
    #[serde(default)]
    pub req_id: Option<String>,
    /// Operation type
    #[serde(default)]
    pub op: Option<String>,
}

/// WebSocket message (data push).
#[derive(Debug, Clone, Deserialize)]
pub struct WsMessage {
    /// Topic name
    pub topic: String,
    /// Message type (snapshot, delta)
    #[serde(rename = "type")]
    pub msg_type: Option<String>,
    /// Timestamp
    pub ts: Option<u64>,
    /// Data payload
    pub data: serde_json::Value,
}

/// WebSocket ping request.
#[derive(Debug, Clone, Serialize)]
pub struct WsPing {
    /// Request ID
    pub req_id: String,
    /// Operation type
    pub op: String,
}

impl WsPing {
    /// Create a new ping message.
    pub fn new() -> Self {
        Self {
            req_id: uuid::Uuid::new_v4().to_string(),
            op: "ping".to_string(),
        }
    }
}

impl Default for WsPing {
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket pong response.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPong {
    /// Success flag
    #[serde(default)]
    pub success: Option<bool>,
    /// Return message
    #[serde(default)]
    pub ret_msg: Option<String>,
    /// Connection ID
    #[serde(default)]
    pub conn_id: Option<String>,
    /// Request ID
    #[serde(default)]
    pub req_id: Option<String>,
    /// Operation type
    #[serde(default)]
    pub op: Option<String>,
}

/// Check if message is a pong response.
pub fn is_pong(msg: &serde_json::Value) -> bool {
    if let Some(op) = msg.get("op").and_then(|v| v.as_str()) {
        return op == "pong";
    }
    if let Some(ret_msg) = msg.get("ret_msg").and_then(|v| v.as_str()) {
        return ret_msg == "pong";
    }
    false
}

/// Check if message is an auth response.
pub fn is_auth_response(msg: &serde_json::Value) -> bool {
    msg.get("op")
        .and_then(|v| v.as_str())
        .map(|op| op == "auth")
        .unwrap_or(false)
}

/// Check if message is a subscription response.
pub fn is_subscription_response(msg: &serde_json::Value) -> bool {
    msg.get("op")
        .and_then(|v| v.as_str())
        .map(|op| op == "subscribe")
        .unwrap_or(false)
}

/// Check if message is a data message.
pub fn is_data_message(msg: &serde_json::Value) -> bool {
    msg.get("topic").is_some()
}
