use std::env;
use std::fs;
use log::debug;

pub fn read_env_or_file(var: &str) -> Result<String, std::io::Error> {
    let file_var = format!("{var}_FILE");
    debug!("[read_env_or_file] Checking for file-based env: {}", file_var);
    if let Ok(path) = env::var(&file_var) {
        debug!("[read_env_or_file] Found {}={}, reading file...", file_var, path);
        let v = fs::read_to_string(&path)?;
        debug!("[read_env_or_file] Read value from file for {}: [REDACTED] ({} bytes)", var, v.trim().len());
        return Ok(v.trim().to_string());
    }

    debug!("[read_env_or_file] {} not set, falling back to env var {}", file_var, var);
    let val = env::var(var).unwrap_or_default();
    debug!("[read_env_or_file] Value for {}: [REDACTED] ({} bytes)", var, val.len());
    Ok(val)
}