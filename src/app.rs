use crate::state::WatchState;
use crate::animation::AnimationController;

/// 主应用结构
/// 简化实现：集成状态管理和动画系统
pub struct WatchApp {
    state: WatchState,
    animation: AnimationController,
}

impl WatchApp {
    /// 创建新的监控应用
    /// 简化实现：初始化状态和动画控制器
    pub fn new() -> Self {
        WatchApp {
            state: WatchState::Stopped,
            animation: AnimationController::new(3),
        }
    }

    /// 处理键盘输入
    /// 简化实现：状态转换逻辑
    pub fn handle_input(&mut self, key: &str) -> bool {
        if let Some(new_state) = self.state.handle_input(key) {
            self.state = new_state;
            true
        } else {
            false
        }
    }

    /// 获取当前状态
    pub fn get_state(&self) -> &WatchState {
        &self.state
    }

    /// 显示当前状态
    /// 简化实现：根据状态显示不同的界面信息
    pub fn display_status(&self) {
        match self.state {
            WatchState::Stopped => {
                println!("🔴 停止状态 - 按 Enter 发送");
            },
            WatchState::Running => {
                let dots = self.animation.get_display();
                println!("⚪ 运行状态{} - 按 Esc 中断", dots);
            },
            WatchState::Interrupted => {
                println!("⚠️ 已中断 - 按任意键继续");
            }
        }
    }

    /// 获取完整的显示字符串
    pub fn get_display_string(&self) -> String {
        match self.state {
            WatchState::Stopped => {
                "🔴 停止状态 - 按 Enter 发送".to_string()
            },
            WatchState::Running => {
                let dots = self.animation.get_display();
                format!("⚪ 运行状态{} - 按 Esc 中断", dots)
            },
            WatchState::Interrupted => {
                "⚠️ 已中断 - 按任意键继续".to_string()
            }
        }
    }

    /// 检查是否在运行状态
    pub fn is_running(&self) -> bool {
        matches!(self.state, WatchState::Running)
    }

    /// 检查是否在停止状态
    pub fn is_stopped(&self) -> bool {
        matches!(self.state, WatchState::Stopped)
    }

    /// 检查是否在中断状态
    pub fn is_interrupted(&self) -> bool {
        matches!(self.state, WatchState::Interrupted)
    }
}

impl Default for WatchApp {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_creation() {
        let app = WatchApp::new();
        assert!(app.is_stopped());
        assert!(!app.is_running());
        assert!(!app.is_interrupted());
    }

    #[test]
    fn test_state_transitions() {
        let mut app = WatchApp::new();
        
        // 测试停止到运行
        assert!(app.handle_input("\n"));
        assert!(app.is_running());
        assert!(!app.is_stopped());
        
        // 测试运行到中断
        assert!(app.handle_input("esc"));
        assert!(app.is_interrupted());
        assert!(!app.is_running());
    }

    #[test]
    fn test_display_strings() {
        let mut app = WatchApp::new();
        
        // 测试停止状态显示
        let stopped_display = app.get_display_string();
        assert!(stopped_display.contains("停止状态"));
        assert!(stopped_display.contains("Enter"));
        
        // 切换到运行状态
        app.handle_input("\n");
        let running_display = app.get_display_string();
        assert!(running_display.contains("运行状态"));
        assert!(running_display.contains("Esc"));
        
        // 切换到中断状态
        app.handle_input("esc");
        let interrupted_display = app.get_display_string();
        assert!(interrupted_display.contains("已中断"));
    }

    #[test]
    fn test_invalid_input_handling() {
        let mut app = WatchApp::new();
        
        // 测试无效输入不改变状态
        assert!(!app.handle_input("invalid"));
        assert!(app.is_stopped());
        
        // 切换到运行状态
        app.handle_input("\n");
        assert!(!app.handle_input("invalid"));
        assert!(app.is_running());
    }
}