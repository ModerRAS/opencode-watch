use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub tmux: TmuxConfig,
    pub llm: LlmConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TmuxConfig {
    pub pane: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmConfig {
    pub backend: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub interval: u64,
    pub stuck_sec: u64,
    pub max_retry: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tmux: TmuxConfig {
                pane: "%18".to_string(),
            },
            llm: LlmConfig {
                backend: "ollama".to_string(),
            },
            monitoring: MonitoringConfig {
                interval: 5,
                stuck_sec: 30,
                max_retry: 3,
            },
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        if !Path::new(path).exists() {
            return Err("配置文件不存在".into());
        }
        
        let content = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = serde_yaml::to_string(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}