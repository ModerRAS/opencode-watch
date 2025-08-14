use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// 动画状态结构
/// 原本实现：静态状态显示
/// 简化实现：动态点动画，提供运行状态的视觉反馈
#[derive(Debug)]
pub struct AnimationState {
    dots_count: usize,
    max_dots: usize,
}

impl AnimationState {
    /// 创建新的动画状态
    /// 简化实现：初始化动画参数
    pub fn new(max_dots: usize) -> Self {
        AnimationState {
            dots_count: 0,
            max_dots,
        }
    }

    /// 更新动画状态
    /// 简化实现：循环更新点的数量（0到max_dots）
    pub fn update(&mut self) {
        self.dots_count = (self.dots_count + 1) % (self.max_dots + 1);
    }

    /// 获取当前动画显示字符串
    /// 简化实现：返回当前数量的点
    pub fn get_display(&self) -> String {
        ".".repeat(self.dots_count)
    }

    /// 获取当前点数
    pub fn get_dots_count(&self) -> usize {
        self.dots_count
    }
}

/// 动画控制器
/// 简化实现：管理动画线程和状态更新
pub struct AnimationController {
    state: Arc<Mutex<AnimationState>>,
}

impl AnimationController {
    /// 创建新的动画控制器
    /// 简化实现：初始化动画状态并启动后台线程
    pub fn new(max_dots: usize) -> Self {
        let state = Arc::new(Mutex::new(AnimationState::new(max_dots)));
        
        // 启动动画线程
        let state_clone = state.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500)); // 每500ms更新一次
                let mut anim = state_clone.lock().unwrap();
                anim.update();
            }
        });
        
        AnimationController { state }
    }

    /// 获取当前动画显示
    /// 简化实现：线程安全地获取动画状态
    pub fn get_display(&self) -> String {
        let anim = self.state.lock().unwrap();
        anim.get_display()
    }

    /// 获取当前点数
    pub fn get_dots_count(&self) -> usize {
        let anim = self.state.lock().unwrap();
        anim.get_dots_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animation_cycle() {
        let mut anim = AnimationState::new(3);
        
        // 初始状态应该是0
        assert_eq!(anim.dots_count, 0);
        
        // 测试0-3的循环
        for i in 1..4 {
            anim.update();
            assert_eq!(anim.dots_count, i);
        }
        
        // 测试循环回到0
        anim.update();
        assert_eq!(anim.dots_count, 0);
    }

    #[test]
    fn test_display_generation() {
        let anim = AnimationState::new(3);
        assert_eq!(anim.get_display(), "");
        
        let mut anim = AnimationState::new(3);
        anim.dots_count = 2;
        assert_eq!(anim.get_display(), "..");
        
        anim.dots_count = 3;
        assert_eq!(anim.get_display(), "...");
    }

    #[test]
    fn test_animation_controller() {
        let controller = AnimationController::new(3);
        let display = controller.get_display();
        
        // 验证显示字符串长度不超过最大点数
        assert!(display.len() <= 3);
        
        // 验证只包含点字符
        assert!(display.chars().all(|c| c == '.'));
    }
}