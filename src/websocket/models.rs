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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTradeAuthRequest {
    pub req_id: String,
    pub op: String,
    pub args: Vec<serde_json::Value>,
}

#[derive(Serialize)]
#[serde(untagged)] // Fondamentale per mantenere il formato JSON originale
pub enum AuthRequest {
    Trade(WsTradeAuthRequest),
    Public(WsAuthRequest),
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTradeAuthResponse {
    pub req_id: String,
    pub ret_code: i32,
    pub ret_msg: String,
    pub op: String,
    pub conn_id: String,
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


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WsTradeOrderHeader {
    #[serde(rename = "X-BAPI-TIMESTAMP")]
    pub x_bapi_timestamp: String, 
    // #[serde(rename = "X-BAPI-RECV-WINDOW")]
    // pub x_bapi_recv_window: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WsTradeOrderCategory {
    Spot,
    Linear,
    Inverse,
    Option,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WsTradeOrderOp {
    Create,
    Amend,
    Delete
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTradeOrderArgs {
    pub category: String, // linear, inverse, spot, option
    pub symbol: String,
    pub side: String,     // Buy, Sell
    pub order_type: String, // Market, Limit
    pub qty: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub is_leverage: Option<i32>, // 0: false, 1: true
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>, // baseCoin, quoteCoin
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub time_in_force: Option<String>, // GTC, IOC, FOK, PostOnly
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub order_link_id: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub take_profit: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub stop_loss: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tp_trigger_by: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sl_trigger_by: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub reduce_only: Option<bool>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub close_on_trigger: Option<bool>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub position_idx: Option<i32>, // 0, 1, 2
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub trigger_price: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub trigger_by: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tp_limit_price: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sl_limit_price: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub tp_order_type: Option<String>,
    
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub sl_order_type: Option<String>,

}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OptionTickerData {
    pub symbol: String,
    pub ask_iv: String,
    pub ask_price: String,
    pub ask_size: String,
    pub bid_iv: String,
    pub bid_price: String,
    
    pub bid_size: String,
    
    pub delta: String,
    
    pub gamma: String,
    
    pub theta: String,
    
    pub vega: String,
    
    pub mark_price: String,
    
    pub index_price: String,
    
    pub underlying_price: String,
    
    pub open_interest: String,
    
    pub volume24h: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTradeOrder {
    pub req_id: uuid::Uuid,
    pub header: WsTradeOrderHeader,
    /// order.create order.amend order.cancel
    pub op: String,
    pub args: Vec<WsTradeOrderArgs>


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
