#!/bin/bash

# 监控%17 pane的守护进程启动脚本
# 使用opencode-watch监控tmux pane %17

# 设置工作目录
cd /root/WorkSpace/Rust/opencode-watch

# 配置参数
PANE="%17"
BACKEND="none"  # 使用none后端，直接干预无需LLM分析
INTERVAL=10     # 10秒检查一次，避免过于频繁
STUCK_SEC=50    # 50秒无变化判定为卡住（5次无变化）
MAX_RETRY=5     # 最大重试次数

# 日志配置
LOG_DIR="/var/log/opencode-watch"
LOG_FILE="${LOG_DIR}/monitor_17.log"
PID_FILE="${LOG_DIR}/monitor_17.pid"

# 创建日志目录
mkdir -p "$LOG_DIR"

# 启动监控
echo "$(date): 启动opencode-watch守护进程，监控pane $PANE" >> "$LOG_FILE"
echo "$(date): 配置 - backend: $BACKEND, interval: ${INTERVAL}s, stuck_sec: ${STUCK_SEC}s" >> "$LOG_FILE"

# 使用pm2启动进程
pm2 start \
  --name "opencode-watch-17" \
  --interpreter none \
  --time \
  --log "$LOG_FILE" \
  --output "${LOG_FILE}.out" \
  --error "${LOG_FILE}.err" \
  --merge-logs \
  ./target/release/opencode-watch \
  -- \
  --pane "$PANE" \
  --backend "$BACKEND" \
  --interval "$INTERVAL" \
  --stuck-sec "$STUCK_SEC" \
  --max-retry "$MAX_RETRY" \
  --config "/root/WorkSpace/Rust/opencode-watch/config.yaml"

echo "$(date): 守护进程已启动，PM2进程名称: opencode-watch-17" >> "$LOG_FILE"
echo "使用以下命令查看状态:"
echo "  pm2 status opencode-watch-17"
echo "  pm2 logs opencode-watch-17"
echo "  ./manage_monitor_17.sh status"