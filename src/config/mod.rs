mod loader;

pub use loader::*;

use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    3000
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: default_host(),
            port: default_port(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointConfig {
    pub path: String,
    #[serde(default = "default_method")]
    pub method: String,
    pub response: ResponseConfig,
    #[serde(default)]
    pub delay: Option<DelayConfig>,
    #[serde(default)]
    pub timeout: bool,
    #[serde(default)]
    pub validation: Option<ValidationConfig>,
    #[serde(default)]
    pub conditions: Vec<ConditionConfig>,
}

fn default_method() -> String {
    "GET".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseConfig {
    #[serde(default = "default_status")]
    pub status: u16,
    #[serde(default)]
    pub headers: HashMap<String, String>,
    pub body: serde_json::Value,
}

fn default_status() -> u16 {
    200
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DelayConfig {
    Fixed(u64),
    Range {
        #[serde(rename = "type")]
        delay_type: Option<String>,
        min: u64,
        max: u64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationConfig {
    #[serde(default)]
    pub params: HashMap<String, ParamValidation>,
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub headers: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamValidation {
    #[serde(default)]
    pub pattern: Option<String>,
    #[serde(default)]
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionConfig {
    #[serde(rename = "if")]
    pub condition: ConditionCheck,
    pub response: ResponseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionCheck {
    #[serde(default)]
    pub param: Option<String>,
    #[serde(default)]
    pub header: Option<String>,
    #[serde(default)]
    pub body_field: Option<String>,
    #[serde(default)]
    pub equals: Option<String>,
    #[serde(default)]
    pub contains: Option<String>,
    #[serde(default)]
    pub matches: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MockConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub endpoints: Vec<EndpointConfig>,
}

impl Default for MockConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            endpoints: Vec::new(),
        }
    }
}

#[allow(dead_code)]
pub struct ConfigManager {
    config: Arc<RwLock<MockConfig>>,
    config_path: String,
}

impl ConfigManager {
    pub async fn new(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let config = load_config(path)?;
        
        Ok(Self {
            config: Arc::new(RwLock::new(config)),
            config_path: path.to_string(),
        })
    }

    pub fn get_config(&self) -> MockConfig {
        self.config.read().clone()
    }

    #[allow(dead_code)]
    pub fn reload(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let new_config = load_config(&self.config_path)?;
        *self.config.write() = new_config;
        tracing::info!("Configuration reloaded successfully");
        Ok(())
    }

    pub fn update_config(&self, new_config: MockConfig) {
        *self.config.write() = new_config;
    }
}
