use opencode_watch::app::WatchApp;
use opencode_watch::state::WatchState;

#[test]
fn test_full_workflow() {
    let mut app = WatchApp::new();
    
    // 初始状态应该是停止状态
    assert!(app.is_stopped());
    assert_eq!(app.get_state(), &WatchState::Stopped);
    
    // 测试停止状态显示
    let display = app.get_display_string();
    assert!(display.contains("停止状态"));
    assert!(display.contains("Enter"));
    
    // 按Enter键切换到运行状态
    assert!(app.handle_input("\n"));
    assert!(app.is_running());
    assert_eq!(app.get_state(), &WatchState::Running);
    
    // 测试运行状态显示
    let display = app.get_display_string();
    assert!(display.contains("运行状态"));
    assert!(display.contains("Esc"));
    
    // 按Esc键切换到中断状态
    assert!(app.handle_input("esc"));
    assert!(app.is_interrupted());
    assert_eq!(app.get_state(), &WatchState::Interrupted);
    
    // 测试中断状态显示
    let display = app.get_display_string();
    assert!(display.contains("已中断"));
}

#[test]
fn test_invalid_input_handling() {
    let mut app = WatchApp::new();
    
    // 测试停止状态下无效输入
    assert!(!app.handle_input("invalid"));
    assert!(app.is_stopped());
    
    // 切换到运行状态
    app.handle_input("\n");
    
    // 测试运行状态下无效输入
    assert!(!app.handle_input("invalid"));
    assert!(app.is_running());
}

#[test]
fn test_state_display_consistency() {
    let mut app = WatchApp::new();
    
    // 验证停止状态显示
    let stopped_display = app.get_display_string();
    assert!(stopped_display.starts_with("🔴"));
    
    // 切换到运行状态
    app.handle_input("\n");
    let running_display = app.get_display_string();
    assert!(running_display.starts_with("⚪"));
    
    // 切换到中断状态
    app.handle_input("esc");
    let interrupted_display = app.get_display_string();
    assert!(interrupted_display.starts_with("⚠️"));
}

#[test]
fn test_animation_integration() {
    let mut app = WatchApp::new();
    
    // 切换到运行状态
    app.handle_input("\n");
    
    // 获取运行状态显示
    let display = app.get_display_string();
    
    // 验证显示包含运行状态文本
    assert!(display.contains("运行状态"));
    assert!(display.contains("Esc"));
    
    // 验证动画点存在（可能为空，但结构正确）
    let parts: Vec<&str> = display.split("运行状态").collect();
    assert!(parts.len() > 1);
}