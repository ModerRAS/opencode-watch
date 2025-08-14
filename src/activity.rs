use std::time::{Duration, Instant};
use crate::tmux::TmuxClient;
use anyhow::Result;

pub struct ActivityDetector {
    tmux_client: TmuxClient,
    last_content: String,
    last_check: Instant,
}

impl ActivityDetector {
    pub fn new() -> Self {
        ActivityDetector {
            tmux_client: TmuxClient::new(),
            last_content: String::new(),
            last_check: Instant::now(),
        }
    }
    
    pub fn check_activity(&mut self, pane: &str) -> Result<bool> {
        let current_content = self.tmux_client.capture_pane_content(pane)?;
        
        let has_activity = current_content != self.last_content;
        
        self.last_content = current_content;
        self.last_check = Instant::now();
        
        Ok(has_activity)
    }
    
    pub fn get_time_since_last_check(&self) -> Duration {
        self.last_check.elapsed()
    }
}