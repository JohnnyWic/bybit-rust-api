//! Authentication and request signing for Bybit API.

use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;

/// Generate HMAC-SHA256 signature for Bybit API requests.
///
/// The signature is calculated as:
/// `HMAC_SHA256(timestamp + api_key + recv_window + payload)`
///
/// # Arguments
/// * `api_secret` - The API secret key
/// * `timestamp` - Current timestamp in milliseconds
/// * `api_key` - The API key
/// * `recv_window` - Receive window in milliseconds
/// * `payload` - Request payload (query string for GET, JSON body for POST)
///
/// # Returns
/// Hex-encoded signature string
pub fn generate_signature(
    api_secret: &str,
    timestamp: u64,
    api_key: &str,
    recv_window: u64,
    payload: &str,
) -> String {
    let param_str = format!("{}{}{}{}", timestamp, api_key, recv_window, payload);

    let mut mac =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(param_str.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

/// Generate current timestamp in milliseconds.
pub fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64
}

/// Generate WebSocket authentication signature.
///
/// For WebSocket private channels, the signature format is:
/// `HMAC_SHA256("GET/realtime" + expires)`
///
/// # Arguments
/// * `api_secret` - The API secret key
/// * `expires` - Expiration timestamp in milliseconds
///
/// # Returns
/// Hex-encoded signature string
pub fn generate_ws_signature(api_secret: &str, expires: u64) -> String {
    let param_str = format!("GET/realtime{}", expires);

    let mut mac =
        HmacSha256::new_from_slice(api_secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(param_str.as_bytes());

    hex::encode(mac.finalize().into_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_signature() {
        // Example from Bybit documentation
        let api_secret = "test_secret";
        let timestamp = 1659582742000u64;
        let api_key = "test_key";
        let recv_window = 5000u64;
        let payload = r#"{"symbol":"BTCUSDT"}"#;

        let signature = generate_signature(api_secret, timestamp, api_key, recv_window, payload);

        // Signature should be 64 hex characters (256 bits)
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_generate_ws_signature() {
        let api_secret = "test_secret";
        let expires = 1659582752000u64;

        let signature = generate_ws_signature(api_secret, expires);

        // Signature should be 64 hex characters
        assert_eq!(signature.len(), 64);
    }

    #[test]
    fn test_get_timestamp() {
        let timestamp = get_timestamp();

        // Should be a reasonable timestamp (after 2020)
        assert!(timestamp > 1577836800000); // 2020-01-01
    }
}
