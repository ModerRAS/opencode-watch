# Opencode-Watch

![GitHub release (latest by date)](https://img.shields.io/github/v/release/ModerRAS/opencode-watch)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/ModerRAS/opencode-watch/test.yml)
![Crates.io](https://img.shields.io/crates/v/opencode-watch)

## 项目概述
Opencode-Watch 是一个智能的 tmux pane 监控工具，专门用于检测 opencode 等终端应用的卡住状态，并提供自动干预功能。当检测到终端长时间无响应时，工具会自动发送继续指令，恢复工作流程。

## 🚀 核心功能

### 智能监控
- **tmux pane 监控**: 实时监控指定 tmux pane 的内容变化
- **卡住状态检测**: 基于内容变化智能识别终端是否卡住
- **自动干预**: 检测到卡住时自动发送"继续"指令和回车
- **LLM 分析**: 支持多种 LLM 后端进行智能状态分析（可选）

### 配置选项
- **灵活配置**: 支持命令行参数和配置文件
- **多 LLM 后端**: 支持 Ollama、OpenAI、OpenRouter、None
- **可调参数**: 监控间隔、卡住判定时间、最大重试次数

### 实时反馈
- **状态显示**: 实时显示监控状态和检测结果
- **内容预览**: 显示捕获的最后几行内容
- **干预日志**: 详细记录干预操作和结果

## 🛠️ 技术架构

### 工作原理

#### 卡住状态检测
opencode-watch 使用基于 opencode 特定标识的精确卡住状态检测：

**原本实现（错误）**：
- 基于画面内容变化来判断活动状态
- 使用LLM分析来确定是否卡住
- 这种方式在opencode中不准确，因为opencode工作时画面可能不变化

**简化实现（正确）**：
- 基于 "working" 和 "esc interrupt" 标识来判断工作状态
- 当检测到 "working" 或 "esc interrupt" 标识时，认为 opencode 正在工作
- 当 "working" 和 "esc interrupt" 标识都消失时，确认 opencode 卡住
- 这种方式直接、准确，符合opencode的实际工作模式

#### 检测逻辑
```rust
// 简化实现：src/activity.rs:18-27
let has_working = current_content.contains("working");
let has_esc_interrupt = current_content.contains("esc interrupt");
let is_active = has_working || has_esc_interrupt;

// 工作状态：有working或esc interrupt标识
// 卡住状态：working和esc interrupt标识都消失
```

#### 干预机制
当检测到卡住状态时，opencode-watch 会按配置的命令列表进行循环干预：

1. **命令循环**：按顺序使用配置的干预命令列表
2. **智能延迟**：支持命令发送延迟和回车键发送延迟
3. **特殊处理**：对 "C-c" 和以 "/" 开头的命令进行特殊处理（不发送回车）

**默认命令列表**：
- "继续" - 基础继续指令
- "继续工作" - 友好提示
- "请继续" - 礼貌请求
- "/compact" - 上下文压缩指令
- "C-c" - 中断当前操作
- "请继续工作" - 礼貌的工作继续请求

### 监控引擎
```rust
// 核心监控逻辑
pub struct TmuxClient;

impl TmuxClient {
    // 捕获 tmux pane 内容
    pub fn capture_pane_content(&self, pane: &str) -> Result<String>
    
    // 发送按键到 pane
    pub fn send_keys(&self, pane: &str, keys: &str) -> Result<()>
    
    // 检查 pane 是否存在
    pub fn check_pane_exists(&self, pane: &str) -> bool
}
```

### 活动检测器
```rust
// 基于 working/esc interrupt 标识的检测
pub struct ActivityDetector;

impl ActivityDetector {
    // 检查工作状态
    pub fn check_activity(&mut self, pane: &str) -> Result<bool>
    
    // 基于标识检测而非内容变化
    // working: true + esc interrupt: true = 工作中
    // working: false + esc interrupt: false = 卡住
}
```

### 主监控循环
```rust
// 主监控逻辑
fn main() -> Result<()> {
    // 配置加载
    let config = load_config();
    
    // 监控循环
    loop {
        // 1. 检查 pane 存在性
        // 2. 捕获内容
        // 3. 检测 working/esc interrupt 标识
        // 4. 判断工作状态
        // 5. 连续检测到卡住时进行干预
        // 6. 按命令列表循环发送干预指令
        // 7. 等待下次检查
    }
}
```

## ✅ 功能特性

### 🎯 核心功能
- [x] **tmux pane 监控**: 实时捕获和分析终端内容
- [x] **精确卡住检测**: 基于 working/esc interrupt 标识识别
- [x] **智能干预**: 按命令列表循环发送干预指令
- [x] **实时状态显示**: 直观的监控反馈和检测结果
- [x] **灵活配置**: 丰富的配置选项和命令行参数
- [x] **命令循环**: 支持自定义干预命令列表和循环机制

### 🔧 技术特性
- [x] **跨平台支持**: Linux、Windows、macOS
- [x] **musl 静态构建**: 无依赖的二进制文件
- [x] **错误处理**: 完善的错误处理机制
- [x] **异步处理**: 基于 Tokio 的异步架构
- [x] **CI/CD 流水线**: 自动测试和发布

### 🧪 测试验证
- **单元测试**: 配置系统测试用例，全部通过
- **集成测试**: 实际环境测试验证，全部通过
- **实际场景测试**: 
  - %17 pane: 检测到working标识消失 → 发送"继续工作" → 恢复working标识 ✅
  - %19 pane: 检测到working标识消失 → 发送"请继续" → 恢复working标识 ✅
  - 命令循环测试：[1/6]"继续" → [2/6]"继续工作" → [3/6]"请继续" ✅

### 🎨 用户体验
- **实时反馈**: 显示捕获内容和检测结果
- **状态指示**: 清晰的运行状态和干预日志
- **易于使用**: 简单的命令行界面
- **可配置性**: 丰富的配置选项

## 🚀 快速开始

### 安装

#### 从 GitHub Releases 安装
```bash
# Linux (musl)
wget https://github.com/ModerRAS/opencode-watch/releases/latest/download/opencode-watch-x86_64-unknown-linux-musl.tar.gz
tar -xzf opencode-watch-x86_64-unknown-linux-musl.tar.gz
sudo mv opencode-watch /usr/local/bin/

# Windows
wget https://github.com/ModerRAS/opencode-watch/releases/latest/download/opencode-watch-x86_64-pc-windows-gnu.zip
unzip opencode-watch-x86_64-pc-windows-gnu.zip

# macOS
wget https://github.com/ModerRAS/opencode-watch/releases/latest/download/opencode-watch-x86_64-apple-darwin.tar.gz
tar -xzf opencode-watch-x86_64-apple-darwin.tar.gz
sudo mv opencode-watch /usr/local/bin/
```

#### 从源码构建
```bash
# 克隆仓库
git clone https://github.com/ModerRAS/opencode-watch.git
cd opencode-watch

# 构建发布版本
cargo build --release

# 安装到系统
sudo cp target/release/opencode-watch /usr/local/bin/
```

### 基本使用

#### 监控指定 pane
```bash
# 监控 %18 pane（默认）
opencode-watch

# 监控指定 pane
opencode-watch --pane %17

# 使用不同的监控间隔
opencode-watch --interval 10

# 使用 LLM 分析
opencode-watch --backend ollama
```

#### 命令行选项
```bash
opencode-watch [OPTIONS]

选项:
  -p, --pane <PANE>          监控的 tmux pane [默认: %18]
  -b, --backend <BACKEND>    LLM 后端 [ollama|openai|openrouter|none] [默认: none]
  -i, --interval <SECONDS>   监控间隔（秒）[默认: 5]
  -s, --stuck-sec <SECONDS>  卡住判定时间（秒）[默认: 30]
  -r, --max-retry <COUNT>    最大重试次数 [默认: 3]
  -c, --config <PATH>        配置文件路径 [默认: config.yaml]
  -h, --help                 显示帮助信息
  -V, --version              显示版本信息
```

#### 配置文件
```yaml
# config.yaml
tmux:
  pane: "%18"              # 监控的 pane

llm:
  backend: "none"          # LLM 后端 [ollama|openai|openrouter|none]

monitoring:
  interval: 5              # 监控间隔（秒）
  stuck_sec: 30            # 卡住判定时间（秒）
  max_retry: 3             # 最大重试次数
  command_delay_ms: 1000   # 指令发送延迟（毫秒）
  enter_delay_ms: 3000     # 回车发送延迟（毫秒）
  
  # 干预指令列表 - 按顺序循环使用
  intervention_commands:
    - "继续"               # 基础继续指令
    - "继续工作"            # 友好提示
    - "请继续"             # 礼貌请求
    - "/compact"           # 上下文压缩指令
    - "重新开始"           # 重新开始工作
    - "恢复工作"            # 恢复工作状态
```

#### 干预指令列表配置
工具支持自定义干预指令列表，每次检测到卡住状态时，会按顺序选择下一条指令执行：

```yaml
# 简单配置
intervention_commands:
  - "继续"
  - "/compact"

# 详细配置
intervention_commands:
  - "继续"                 # 基础继续指令
  - "继续工作"              # 友好提示
  - "请继续"               # 礼貌请求
  - "/compact"             # 上下文压缩
  - "重新开始"             # 重新开始工作
  - "恢复工作"              # 恢复状态
  - "继续执行"              # 继续任务
  - "请继续工作"            # 礼貌请求

# 针对不同项目的配置
intervention_commands:
  - "continue"
  - "please continue"
  - "/compact"
  - "restart"
```

#### 指令执行机制
1. **循环执行**: 指令按列表顺序循环使用
2. **延迟配置**: 
   - `command_delay_ms`: 指令输入完成后的等待时间
   - `enter_delay_ms`: 回车键发送的总延迟时间
3. **特殊指令**: 以`/`开头的指令为特殊命令（如`/compact`）
4. **自动回车**: 所有指令都会自动发送回车键执行

#### 推荐配置场景
```yaml
# 高频率监控（快速响应）
monitoring:
  interval: 5
  stuck_sec: 20
  command_delay_ms: 800
  enter_delay_ms: 2000

# 标准监控（平衡性能）
monitoring:
  interval: 8
  stuck_sec: 30
  command_delay_ms: 1000
  enter_delay_ms: 3000

# 低频率监控（减少干扰）
monitoring:
  interval: 15
  stuck_sec: 60
  command_delay_ms: 1500
  enter_delay_ms: 4000
```

### LLM 配置

#### Ollama（默认）
```bash
# 启动 Ollama 服务
ollama serve

# 拉取模型
ollama pull llama3.2

# 使用 Ollama 后端
opencode-watch --backend ollama
```

#### OpenAI
```bash
# 设置 API Key
export OPENAI_API_KEY="your-openai-api-key"

# 使用 OpenAI 后端
opencode-watch --backend openai
```

#### OpenRouter
```bash
# 设置 API Key
export OPENROUTER_API_KEY="your-openrouter-api-key"

# 使用 OpenRouter 后端
opencode-watch --backend openrouter
```

### 开发和测试
```bash
# 克隆仓库
git clone https://github.com/ModerRAS/opencode-watch.git
cd opencode-watch

# 开发模式运行
cargo run

# 运行测试
cargo test

# 检查代码格式
cargo fmt --check

# 运行 clippy
cargo clippy

# 构建发布版本
cargo build --release
```

## 📋 路线图

### 🎯 短期目标
- [ ] **多 pane 监控**: 支持同时监控多个 pane
- [ ] **日志系统**: 添加详细的日志记录和轮转
- [ ] **配置热重载**: 支持运行时配置更新
- [ ] **性能优化**: 减少资源占用和提升响应速度

### 🚀 中期目标
- [ ] **Web 界面**: 提供 Web 监控面板
- [ ] **通知系统**: 集成邮件、Slack 等通知方式
- [ ] **插件系统**: 支持自定义干预策略
- [ ] **数据分析**: 提供卡住模式分析和报告

### 🔮 长期目标
- [ ] **机器学习**: 基于 ML 的智能卡住预测
- [ ] **分布式支持**: 支持多主机监控
- [ ] **API 接口**: 提供 REST API 集成
- [ ] **云端部署**: 支持 Kubernetes 部署

## 🤝 贡献指南

### 开发环境设置
```bash
# 1. 克隆仓库
git clone https://github.com/ModerRAS/opencode-watch.git
cd opencode-watch

# 2. 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 3. 安装依赖
cargo build

# 4. 运行测试
cargo test
```

### 贡献流程
1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

### 代码规范
- 遵循 Rust 官方代码风格
- 添加适当的测试用例
- 更新相关文档
- 确保 CI/CD 通过

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 🙏 致谢

- [tmux](https://github.com/tmux/tmux) - 终端复用器
- [Tokio](https://tokio.rs/) - Rust 异步运行时
- [Crossterm](https://github.com/crossterm-rs/crossterm) - 终端操作库
- [Ollama](https://ollama.com/) - 本地 LLM 运行时

## 📞 支持

- 🐛 **问题反馈**: [GitHub Issues](https://github.com/ModerRAS/opencode-watch/issues)
- 💬 **功能讨论**: [GitHub Discussions](https://github.com/ModerRAS/opencode-watch/discussions)
- 📧 **邮件联系**: [创建 Issue](https://github.com/ModerRAS/opencode-watch/issues/new)

---

**⭐ 如果这个项目对您有帮助，请给个 Star 支持一下！**

## 🏗️ 项目结构

```
opencode-watch/
├── src/
│   ├── main.rs              # 主程序入口
│   ├── config.rs            # 配置管理
│   ├── args.rs              # 命令行参数
│   ├── tmux.rs              # tmux 客户端
│   ├── activity.rs          # 活动检测
│   ├── llm.rs               # LLM 客户端
│   ├── monitor.rs           # 监控循环
│   ├── state.rs             # 状态管理
│   ├── animation.rs         # 动画系统
│   ├── app.rs               # 应用管理
│   └── lib.rs               # 库入口
├── tests/
│   └── integration_test.rs  # 集成测试
├── .github/
│   └── workflows/
│       ├── test.yml         # 测试工作流
│       └── release.yml      # 发布工作流
├── Cargo.toml               # 项目配置
├── Cargo.lock               # 依赖锁定
├── LICENSE                  # MIT 许可证
├── README.md                # 项目文档
└── demo.sh                  # 演示脚本
```

## 📦 依赖项

### 核心依赖
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }  # 异步运行时
crossterm = "0.27"                                # 终端操作
clap = { version = "4", features = ["derive"] }   # 命令行解析
serde = { version = "1.0", features = ["derive"] } # 序列化
serde_yaml = "0.9"                                # YAML 配置
anyhow = "1.0"                                    # 错误处理
```

### LLM 依赖
```toml
ollama-rs = "0.3.2"      # Ollama 客户端
reqwest = { version = "0.12", features = ["json"] }  # HTTP 客户端
```

### 开发依赖
```toml
[dev-dependencies]
tokio-test = "0.4"      # 异步测试
```

## 🔧 构建和部署

### 本地构建
```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 代码格式化
cargo fmt

# 静态检查
cargo clippy
```

### 交叉编译
```bash
# Linux musl (静态链接）
cargo build --release --target x86_64-unknown-linux-musl

# Windows
cargo build --release --target x86_64-pc-windows-gnu

# macOS
cargo build --release --target x86_64-apple-darwin
```

### Docker 部署
```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache tmux
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/opencode-watch /usr/local/bin/
CMD ["opencode-watch"]
```

## 🎯 使用场景

### 1. 开发环境监控
```bash
# 监控开发环境的 opencode 进程
opencode-watch --pane %17 --backend ollama
```

### 2. CI/CD 集成
```yaml
# GitHub Actions 示例
- name: Start monitoring
  run: |
    opencode-watch --pane %18 --backend none --interval 10 &
    MONITOR_PID=$!
    
    # 运行构建
    cargo build --release
    
    # 停止监控
    kill $MONITOR_PID
```

### 3. 批量处理
```bash
# 监控多个 pane
for pane in %17 %18 %19; do
  opencode-watch --pane $pane --backend none &
done
wait
```

## ⚠️ 注意事项

### 使用限制
- 需要安装并运行 tmux
- LLM 功能需要相应的服务或 API Key
- 监控间隔不宜过短，避免性能问题

### 最佳实践
- 首次使用建议先使用 `--backend none` 测试
- 根据实际需求调整监控间隔和卡住判定时间
- 定期检查日志文件，了解监控状态

### 故障排除
```bash
# 检查 tmux pane 是否存在
tmux list-panes -a

# 测试 tmux 命令
tmux capture-pane -p -t %18

# 检查 LLM 连接
curl http://localhost:11434/api/tags
```