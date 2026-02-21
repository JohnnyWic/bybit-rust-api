//! Error types for the Bybit API client.

use thiserror::Error;

/// The main error type for the Bybit API client.
///
/// All fallible operations return `Result<T, BybitError>`.
#[derive(Debug, Error)]
pub enum BybitError {
    /// HTTP request failed (network error, timeout, etc.)
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(Box<tokio_tungstenite::tungstenite::Error>),

    /// Request timed out
    #[error("Request timeout")]
    Timeout,

    /// Bybit API returned an error response
    #[error("API error: code={code}, msg={msg}")]
    Api {
        /// Error code from Bybit API
        code: i32,
        /// Error message from Bybit API
        msg: String,
    },

    /// Failed to parse response
    #[error("Parse error: {0}")]
    Parse(String),

    /// Missing required field in response
    #[error("Missing field: {0}")]
    MissingField(&'static str),

    /// Invalid parameter provided
    #[error("Invalid parameter: {0}")]
    InvalidParam(String),

    /// Authentication error (missing or invalid credentials)
    #[error("Authentication error: {0}")]
    Auth(String),
}

/// Result type alias for Bybit operations
pub type Result<T> = std::result::Result<T, BybitError>;

impl BybitError {
    /// Create an API error from code and message
    pub fn api(code: i32, msg: impl Into<String>) -> Self {
        Self::Api {
            code,
            msg: msg.into(),
        }
    }

    /// Create a parse error
    pub fn parse(msg: impl Into<String>) -> Self {
        Self::Parse(msg.into())
    }

    /// Create an invalid parameter error
    pub fn invalid_param(msg: impl Into<String>) -> Self {
        Self::InvalidParam(msg.into())
    }

    /// Check if this is a rate limit error (code 10006)
    pub fn is_rate_limited(&self) -> bool {
        matches!(self, Self::Api { code: 10006, .. })
    }

    /// Check if this is a timeout error
    pub fn is_timeout(&self) -> bool {
        matches!(self, Self::Timeout)
    }
}
