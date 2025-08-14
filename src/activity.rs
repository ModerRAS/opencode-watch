use std::time::{Duration, Instant};
use crate::tmux::TmuxClient;
use anyhow::Result;

pub struct ActivityDetector {
    tmux_client: TmuxClient,
    last_check: Instant,
}

impl ActivityDetector {
    pub fn new() -> Self {
        ActivityDetector {
            tmux_client: TmuxClient::new(),
            last_check: Instant::now(),
        }
    }
    
    pub fn check_activity(&mut self, pane: &str) -> Result<bool> {
        let current_content = self.tmux_client.capture_pane_content(pane)?;
        
        // 检查是否有working或esc interrupt标识
        // 在opencode中，这些标识表示正在工作状态
        let has_working = current_content.contains("working");
        let has_esc_interrupt = current_content.contains("esc interrupt");
        
        // 如果有working或esc interrupt标识，说明正在工作，有活动
        let is_active = has_working || has_esc_interrupt;
        
        self.last_check = Instant::now();
        
        Ok(is_active)
    }
    
    pub fn get_time_since_last_check(&self) -> Duration {
        self.last_check.elapsed()
    }
}