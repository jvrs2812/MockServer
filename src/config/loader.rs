use std::fs;
use std::path::Path;

use super::MockConfig;

pub fn load_config(path: &str) -> Result<MockConfig, Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(path);
    
    if !path.exists() {
        return Err(format!("Config file not found: {}", path.display()).into());
    }

    let content = fs::read_to_string(path)?;
    
    let config: MockConfig = match path.extension().and_then(|e| e.to_str()) {
        Some("yaml") | Some("yml") => serde_yaml::from_str(&content)?,
        Some("json") => serde_json::from_str(&content)?,
        _ => return Err("Unsupported config format. Use .yaml, .yml or .json".into()),
    };

    tracing::info!("Loaded {} endpoints from config", config.endpoints.len());
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_yaml_config() {
        let yaml = r#"
server:
  port: 8080
endpoints:
  - path: "/test"
    method: GET
    response:
      status: 200
      body:
        message: "hello"
"#;
        let config: MockConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.endpoints.len(), 1);
    }
}
