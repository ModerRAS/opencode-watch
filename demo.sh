#!/bin/bash

# Opencode-Watch 演示脚本
# 展示状态管理和动画效果

echo "=== Opencode-Watch 功能演示 ==="
echo

# 构建项目
echo "1. 构建项目..."
cargo build --release
if [ $? -eq 0 ]; then
    echo "✅ 构建成功"
else
    echo "❌ 构建失败"
    exit 1
fi

echo

# 运行测试
echo "2. 运行测试..."
cargo test
if [ $? -eq 0 ]; then
    echo "✅ 所有测试通过"
else
    echo "❌ 测试失败"
    exit 1
fi

echo

# 显示帮助信息
echo "3. 显示帮助信息..."
./target/release/opencode-watch --help

echo

echo "=== 演示说明 ==="
echo "现在可以运行应用进行交互式演示："
echo
echo "运行命令: ./target/release/opencode-watch"
echo
echo "交互说明："
echo "- 初始状态: 🔴 停止状态 - 按 Enter 发送"
echo "- 按 Enter: 切换到运行状态，显示动态点动画"
echo "- 运行状态: ⚪ 运行状态... - 按 Esc 中断"
echo "- 按 Esc: 切换到中断状态"
echo "- 中断状态: ⚠️ 已中断 - 按任意键继续"
echo "- 按 Ctrl+C: 退出应用"
echo
echo "动画效果：运行状态下点数会从0到3循环变化，每500ms更新一次"
echo

echo "=== 项目结构 ==="
echo "src/"
echo "├── main.rs          # 主程序入口"
echo "├── lib.rs           # 库文件入口"
echo "├── state.rs         # 状态管理模块"
echo "├── animation.rs     # 动画系统模块"
echo "├── app.rs           # 主应用模块"
echo "├── config.rs        # 配置管理模块"
echo "├── tmux.rs          # Tmux客户端模块"
echo "├── activity.rs      # 活动检测模块"
echo "├── llm.rs           # LLM客户端模块"
echo "├── monitor.rs       # 监控循环模块"
echo "└── args.rs          # 命令行参数模块"
echo
echo "tests/"
echo "└── integration_test.rs  # 集成测试"
echo

echo "=== 技术特性 ==="
echo "✅ 状态管理: 枚举状态，清晰的状态转换"
echo "✅ 动画系统: 独立线程，动态点动画"
echo "✅ 键盘交互: 原始终端模式，实时响应"
echo "✅ 模块化设计: 清晰的模块分离"
echo "✅ 测试覆盖: 单元测试和集成测试"
echo "✅ 错误处理: 完善的错误处理机制"
echo

echo "演示完成！现在可以运行应用体验功能。"