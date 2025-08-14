#!/bin/bash

# %19 pane监控管理脚本
# 使用方法: ./manage_monitor_19.sh [start|stop|restart|status|logs]

PANEL_NAME="opencode-watch-19"
LOG_DIR="/var/log/opencode-watch"

case "$1" in
    "start")
        echo "启动%19 pane监控..."
        if pm2 status | grep -q "$PANEL_NAME.*online"; then
            echo "监控进程已经在运行"
            pm2 status "$PANEL_NAME"
        else
            ./start_monitor_19.sh
        fi
        ;;
        
    "stop")
        echo "停止%19 pane监控..."
        pm2 stop "$PANEL_NAME"
        pm2 delete "$PANEL_NAME"
        echo "监控进程已停止"
        ;;
        
    "restart")
        echo "重启%19 pane监控..."
        pm2 restart "$PANEL_NAME"
        echo "监控进程已重启"
        ;;
        
    "status")
        echo "=== %19 pane监控状态 ==="
        pm2 status "$PANEL_NAME"
        echo ""
        echo "=== 最近日志 ==="
        pm2 logs "$PANEL_NAME" --lines 10 --out
        ;;
        
    "logs")
        echo "=== %19 pane监控实时日志 ==="
        echo "按 Ctrl+C 退出日志查看"
        pm2 logs "$PANEL_NAME"
        ;;
        
    "monit")
        echo "启动pm2监控界面..."
        pm2 monit
        ;;
        
    *)
        echo "使用方法: $0 [start|stop|restart|status|logs|monit]"
        echo ""
        echo "命令说明:"
        echo "  start   - 启动监控进程"
        echo "  stop    - 停止监控进程"
        echo "  restart - 重启监控进程"
        echo "  status  - 查看进程状态和最近日志"
        echo "  logs    - 查看实时日志"
        echo "  monit   - 启动pm2监控界面"
        echo ""
        echo "当前状态:"
        pm2 status "$PANEL_NAME"
        exit 1
        ;;
esac