use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub tmux: TmuxConfig,
    pub monitoring: MonitoringConfig,
    pub intervention: InterventionConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmuxConfig {
    pub pane: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub interval: u64,
    pub stuck_sec: u64,
    pub max_retry: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterventionConfig {
    #[serde(default = "default_commands")]
    pub commands: Vec<String>,
    #[serde(default = "default_command_delay")]
    pub command_delay_ms: u64,
    #[serde(default = "default_enter_delay")]
    pub enter_delay_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tmux: TmuxConfig {
                pane: "%18".to_string(),
            },
            monitoring: MonitoringConfig {
                interval: 5,
                stuck_sec: 30,
                max_retry: 6, // 增加到6次以支持完整的命令循环
            },
            intervention: InterventionConfig {
                commands: vec![
                    "继续".to_string(),
                    "继续工作".to_string(),
                    "请继续".to_string(),
                    "/compact".to_string(),
                    "C-c".to_string(),
                    "请继续工作".to_string(),
                ],
                command_delay_ms: 1000,
                enter_delay_ms: 3000,
            },
        }
    }
}

// 默认干预指令列表
fn default_commands() -> Vec<String> {
    vec![
        "继续".to_string(),
        "继续工作".to_string(),
        "请继续".to_string(),
        "/compact".to_string(),
        "C-c".to_string(),
        "请继续工作".to_string(),
    ]
}

// 默认指令发送延迟（毫秒）
fn default_command_delay() -> u64 {
    1000
}

// 默认回车发送延迟（毫秒）
fn default_enter_delay() -> u64 {
    3000
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
    
    // 获取下一个干预指令（循环使用）
    pub fn get_next_intervention_command(&self, current_index: usize) -> (String, usize) {
        if self.intervention.commands.is_empty() {
            return ("继续".to_string(), 0);
        }
        
        let next_index = (current_index + 1) % self.intervention.commands.len();
        let command = self.intervention.commands[next_index].clone();
        (command, next_index)
    }
}

// 全局配置缓存
lazy_static! {
    static ref GLOBAL_CONFIG: std::sync::RwLock<Option<Config>> = std::sync::RwLock::new(None);
}

impl Config {
    // 加载全局配置（带缓存）
    pub fn load_global(path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let config = Self::load(path)?;
        let mut global_config = GLOBAL_CONFIG.write().unwrap();
        *global_config = Some(config);
        Ok(())
    }
    
    // 获取全局配置
    pub fn get_global() -> Option<Config> {
        let global_config = GLOBAL_CONFIG.read().unwrap();
        global_config.clone()
    }
    
    // 更新全局配置中的指令索引
    pub fn update_command_index(_new_index: usize) {
        if let Some(_config) = Self::get_global() {
            // 这里可以保存索引到文件或内存中
            // 暂时用简单的内存存储
        }
    }
}