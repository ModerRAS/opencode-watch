use std::fmt;

/// 监控状态枚举
/// 原本实现：简单的布尔状态
/// 简化实现：使用枚举状态管理，提供更清晰的状态转换
#[derive(Debug, Clone, PartialEq)]
pub enum WatchState {
    Stopped,          // 停止状态
    Running,         // 运行状态
    Interrupted,     // 中断状态
}

impl WatchState {
    /// 处理键盘输入并返回新的状态
    /// 简化实现：状态转换处理逻辑
    pub fn handle_input(&self, key: &str) -> Option<Self> {
        match self {
            WatchState::Stopped => {
                if key == "\n" {
                    Some(WatchState::Running)
                } else {
                    None
                }
            },
            WatchState::Running => {
                if key == "esc" {
                    Some(WatchState::Interrupted)
                } else {
                    None
                }
            },
            WatchState::Interrupted => {
                // 中断状态下可以添加恢复逻辑
                None
            }
        }
    }
}

impl fmt::Display for WatchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WatchState::Stopped => write!(f, "🔴 停止状态"),
            WatchState::Running => write!(f, "⚪ 运行状态"),
            WatchState::Interrupted => write!(f, "⚠️ 已中断"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stopped_to_running() {
        let state = WatchState::Stopped;
        if let Some(new_state) = state.handle_input("\n") {
            assert_eq!(new_state, WatchState::Running);
        } else {
            panic!("Expected state transition from Stopped to Running");
        }
    }

    #[test]
    fn test_running_to_interrupted() {
        let state = WatchState::Running;
        if let Some(new_state) = state.handle_input("esc") {
            assert_eq!(new_state, WatchState::Interrupted);
        } else {
            panic!("Expected state transition from Running to Interrupted");
        }
    }

    #[test]
    fn test_invalid_transitions() {
        let state = WatchState::Stopped;
        assert!(state.handle_input("x").is_none());
        
        let state = WatchState::Running;
        assert!(state.handle_input("a").is_none());
    }
}