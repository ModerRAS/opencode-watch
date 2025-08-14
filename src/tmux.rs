use std::process::Command;

pub struct TmuxClient;

impl TmuxClient {
    pub fn new() -> Self {
        TmuxClient
    }
    
    pub fn capture_pane_content(&self, pane: &str) -> Result<String, Box<dyn std::error::Error>> {
        let output = Command::new("tmux")
            .args(&["capture-pane", "-p", "-t", pane])
            .output()?;
            
        if !output.status.success() {
            return Err(format!("tmux capture-pane failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        Ok(String::from_utf8(output.stdout)?)
    }
    
    pub fn send_keys(&self, pane: &str, keys: &str) -> Result<(), Box<dyn std::error::Error>> {
        let output = Command::new("tmux")
            .args(&["send-keys", "-t", pane, keys])
            .output()?;
            
        if !output.status.success() {
            return Err(format!("tmux send-keys failed: {}", String::from_utf8_lossy(&output.stderr)).into());
        }
        
        Ok(())
    }
    
    pub fn check_pane_exists(&self, pane: &str) -> bool {
        Command::new("tmux")
            .args(&["list-panes", "-a"])
            .output()
            .map(|output| {
                let output_str = String::from_utf8_lossy(&output.stdout);
                output_str.contains(pane)
            })
            .unwrap_or(false)
    }
}