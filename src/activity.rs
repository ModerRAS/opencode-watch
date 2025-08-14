use std::time::{Duration, Instant};
use crate::tmux::TmuxClient;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct ActivityResult {
    pub is_active: bool,           // 是否有活动（基于working标识）
    pub has_working: bool,        // 是否有working标识
    pub has_esc_interrupt: bool,  // 是否有esc interrupt标识
    pub content_changed: bool,     // 内容是否有变化
    pub last_content: String,     // 当前内容
}

pub struct ActivityDetector {
    tmux_client: TmuxClient,
    last_content: String,
    last_content_change: Instant,
    last_check: Instant,
}

impl ActivityDetector {
    pub fn new() -> Self {
        ActivityDetector {
            tmux_client: TmuxClient::new(),
            last_content: String::new(),
            last_content_change: Instant::now(),
            last_check: Instant::now(),
        }
    }
    
    /// 第一阶段检测：基于working标识的快速检测
    /// 返回详细的检测结果，包含多个维度的信息
    pub fn check_activity(&mut self, pane: &str) -> Result<ActivityResult> {
        let current_content = self.tmux_client.capture_pane_content(pane)?;
        
        // 检查是否有working或esc interrupt标识
        let has_working = current_content.contains("working");
        let has_esc_interrupt = current_content.contains("esc interrupt");
        
        // 基于标识的活动状态（快速检测）
        let is_active = has_working || has_esc_interrupt;
        
        // 检查内容是否有变化（慢速检测用）
        let content_changed = current_content != self.last_content;
        
        // 如果内容有变化，更新最后变化时间
        if content_changed {
            self.last_content_change = Instant::now();
        }
        
        // 保存当前内容
        self.last_content = current_content.clone();
        self.last_check = Instant::now();
        
        Ok(ActivityResult {
            is_active,
            has_working,
            has_esc_interrupt,
            content_changed,
            last_content: current_content,
        })
    }
    
    /// 第二阶段检测：检查长时间无内容变化
    /// 用于检测"假工作状态"（有working标识但实际卡住）
    pub fn check_long_stuck(&self, duration_threshold: Duration) -> bool {
        let no_change_duration = self.last_content_change.elapsed();
        no_change_duration > duration_threshold
    }
    
    /// 获取自上次内容变化以来的时间
    pub fn get_time_since_last_change(&self) -> Duration {
        self.last_content_change.elapsed()
    }
    
    /// 获取自上次检查以来的时间
    pub fn get_time_since_last_check(&self) -> Duration {
        self.last_check.elapsed()
    }
}