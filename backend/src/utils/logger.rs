use log::{info, warn};

/// Logs a request to the backend.
///
/// # Arguments
/// - `client_ip`: The IP address of the client making the request.
/// - `endpoint`: The endpoint being accessed.
pub fn log_request(client_ip: &str, endpoint: &str) {
    info!("Request from {} to endpoint {}", client_ip, endpoint);
}

/// Logs suspicious activity detected in the backend.
///
/// # Arguments
/// - `client_ip`: The IP address of the client exhibiting suspicious behavior.
/// - `reason`: The reason for flagging the activity as suspicious.
pub fn log_suspicious_activity(client_ip: &str, reason: &str) {
    warn!("Suspicious activity detected from {}: {}", client_ip, reason);
}