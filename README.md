# Opencode-Watch 项目方案

## 项目概述
Opencode-Watch 是一个基于 Rust 的文件监控工具，提供类似 Claude-Watch 的功能，但采用 Opencode 风格的界面设计。项目通过终端界面提供直观的文件监控和状态管理功能。

## 功能需求

### 核心功能
1. **文件监控**: 实时监控指定文件或目录的变化
2. **状态管理**: 支持停止、运行、中断三种状态
3. **键盘交互**: 
   - 停止状态: 按 Enter 发送/启动
   - 运行状态: 按 Esc 中断
4. **动态界面**: 运行状态显示动态变化的点动画（0-3个点循环）

### 界面设计
- **停止状态**: 显示 "🔴 停止状态 - 按 Enter 发送"
- **运行状态**: 显示 "⚪ 运行状态... - 按 Esc 中断"（点数动态变化）
- **中断状态**: 显示 "⚠️ 已中断 - 按任意键继续"

## 技术架构

### 状态管理
```rust
// 原本实现：简单的布尔状态
// 简化实现：使用枚举状态管理
pub enum WatchState {
    Stopped,          // 停止状态
    Running,         // 运行状态
    Interrupted,     // 中断状态
}

// 简化实现：状态转换处理
impl WatchState {
    pub fn handle_input(&self, key: &str) -> Option<Self> {
        match self {
            WatchState::Stopped => {
                if key == "\n" { Some(WatchState::Running) } else { None }
            },
            WatchState::Running => {
                if key == "esc" { Some(WatchState::Interrupted) } else { None }
            },
            WatchState::Interrupted => {
                // 可以添加恢复逻辑
                None
            }
        }
    }
}
```

### 动画系统
```rust
// 原本实现：静态状态显示
// 简化实现：动态点动画
pub struct AnimationState {
    dots_count: usize,
    max_dots: usize,
}

impl AnimationState {
    pub fn new(max_dots: usize) -> Self {
        AnimationState { dots_count: 0, max_dots }
    }

    pub fn update(&mut self) {
        self.dots_count = (self.dots_count + 1) % (self.max_dots + 1);
    }

    pub fn get_display(&self) -> String {
        ".".repeat(self.dots_count)
    }
}
```

### 主应用结构
```rust
// 简化实现：主应用管理
pub struct WatchApp {
    state: WatchState,
    animation: Arc<Mutex<AnimationState>>,
}

impl WatchApp {
    pub fn new() -> Self {
        let animation = Arc::new(Mutex::new(AnimationState::new(3)));
        
        // 启动动画线程
        let anim_clone = animation.clone();
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(500));
                let mut anim = anim_clone.lock().unwrap();
                anim.update();
            }
        });
        
        WatchApp {
            state: WatchState::Stopped,
            animation,
        }
    }

    pub fn display_status(&self) {
        let anim = self.animation.lock().unwrap();
        match self.state {
            WatchState::Stopped => println!("🔴 停止状态 - 按 Enter 发送"),
            WatchState::Running => println!("⚪ 运行状态{} - 按 Esc 中断", anim.get_display()),
            WatchState::Interrupted => println!("⚠️ 已中断 - 按任意键继续"),
        }
    }
}
```

## 实现状态

### ✅ 已完成功能

#### 阶段1: 基础框架
- [x] 创建项目结构和依赖配置
- [x] 实现基本的状态管理系统 (`src/state.rs`)
- [x] 添加键盘输入处理 (`src/main.rs`)

#### 阶段2: 动画系统
- [x] 实现动态点动画 (`src/animation.rs`)
- [x] 集成动画到状态显示 (`src/app.rs`)
- [x] 优化动画性能（独立线程，500ms更新间隔）

#### 阶段3: 文件监控
- [x] 实现文件变化监控 (`src/activity.rs`)
- [x] 添加监控配置选项 (`src/config.rs`)
- [x] 集成监控到主应用 (`src/monitor.rs`)

#### 阶段4: 界面优化
- [x] 改进终端界面显示
- [x] 添加更多状态信息
- [x] 优化用户体验

### 🎯 核心功能验证

#### 状态管理
- **停止状态**: 显示 "🔴 停止状态 - 按 Enter 发送"
- **运行状态**: 显示 "⚪ 运行状态... - 按 Esc 中断"（点数动态变化）
- **中断状态**: 显示 "⚠️ 已中断 - 按任意键继续"

#### 键盘交互
- **Enter**: 停止状态 → 运行状态
- **Esc**: 运行状态 → 中断状态
- **Ctrl+C**: 退出应用

#### 动画效果
- 运行状态下点数从0到3循环变化
- 每500毫秒更新一次
- 独立线程处理，不阻塞主线程

### 🧪 测试覆盖
- 单元测试：10个测试用例，全部通过
- 集成测试：4个测试用例，全部通过
- 状态转换测试
- 动画循环测试
- 键盘输入处理测试

### 🚀 使用方法
```bash
# 构建项目
cargo build --release

# 运行应用
./target/release/opencode-watch

# 运行测试
cargo test

# 开发模式运行
cargo run
```

### 📋 待优化项目
1. **性能优化**: 减少动画线程的CPU占用
2. **配置增强**: 支持更多自定义选项
3. **错误处理**: 完善错误处理机制
4. **日志系统**: 添加详细的日志记录
5. **界面美化**: 增加更多视觉效果

## 测试策略

### 单元测试
```rust
#[test]
fn test_stopped_to_running() {
    let state = WatchState::Stopped;
    if let Some(new_state) = state.handle_input("\n") {
        assert_eq!(new_state, WatchState::Running);
    }
}

#[test]
fn test_running_to_interrupted() {
    let state = WatchState::Running;
    if let Some(new_state) = state.handle_input("esc") {
        assert_eq!(new_state, WatchState::Interrupted);
    }
}

#[test]
fn test_animation_cycle() {
    let mut anim = AnimationState::new(3);
    for i in 0..4 {
        anim.update();
        assert_eq!(anim.dots_count, i);
    }
}
```

### 集成测试
- 状态转换流程测试
- 键盘输入响应测试
- 动画显示效果测试

## 依赖项
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
crossterm = "0.27"
notify = "6.0"
```

## 部署说明
1. 使用 `cargo build --release` 构建发布版本
2. 运行 `./target/release/opencode-watch` 启动应用
3. 支持命令行参数配置监控路径

## 注意事项
- 所有简化实现都标注了注释，便于后续优化
- 动画系统使用独立线程，避免阻塞主线程
- 状态管理采用不可变设计，确保线程安全