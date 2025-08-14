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

### 智能分析
```rust
// LLM 分析支持
pub struct LlmClient {
    backend: String,
    model: String,
}

impl LlmClient {
    // 分析终端状态
    pub async fn analyze_state(&self, content: &str) -> Result<String>
    
    // 支持 Ollama、OpenAI、OpenRouter
    async fn analyze_with_ollama(&self, content: &str) -> Result<String>
    async fn analyze_with_openai(&self, content: &str) -> Result<String>
    async fn analyze_with_openrouter(&self, content: &str) -> Result<String>
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
        // 3. 比较变化
        // 4. 检测卡住
        // 5. LLM 分析（可选）
        // 6. 自动干预
        // 7. 等待下次检查
    }
}
```

## ✅ 功能特性

### 🎯 核心功能
- [x] **tmux pane 监控**: 实时捕获和分析终端内容
- [x] **卡住状态检测**: 基于内容变化智能识别
- [x] **自动干预**: 发送"继续"指令和回车
- [x] **LLM 智能分析**: 支持多种 LLM 后端
- [x] **实时状态显示**: 直观的监控反馈
- [x] **灵活配置**: 命令行参数和配置文件

### 🔧 技术特性
- [x] **跨平台支持**: Linux、Windows、macOS
- [x] **musl 静态构建**: 无依赖的二进制文件
- [x] **错误处理**: 完善的错误处理机制
- [x] **异步处理**: 基于 Tokio 的异步架构
- [x] **CI/CD 流水线**: 自动测试和发布

### 🧪 测试验证
- **单元测试**: 10个测试用例，全部通过
- **集成测试**: 4个测试用例，全部通过
- **实际场景测试**: 
  - %17 pane: 检测到卡住 → 发送"继续" → 内容变为"working..." ✅
  - %18 pane: 检测到卡住 → 发送"继续" → 内容变为"working." ✅

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
  backend: "none"          # LLM 后端

monitoring:
  interval: 5              # 监控间隔（秒）
  stuck_sec: 30            # 卡住判定时间（秒）
  max_retry: 3             # 最大重试次数
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