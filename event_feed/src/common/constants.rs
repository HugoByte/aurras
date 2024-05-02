/// Returns the default ssb server port.
pub fn default_port() -> String {
    "8008".to_string()
}

/// Returns the default ssb server host ip address.
pub fn default_host() -> String {
    "0.0.0.0".to_string()
}

/// Returns the default ssb secret file path.
pub fn default_secret() -> String {
    let home_dir = dirs::home_dir().unwrap();
    format!("{}/.ssb/secret", home_dir.to_string_lossy())
}
