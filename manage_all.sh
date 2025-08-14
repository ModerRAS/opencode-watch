#!/bin/bash

# 所有pane监控管理脚本
# 使用方法: ./manage_all.sh [start|stop|restart|status|logs]

LOG_DIR="/var/log/opencode-watch"

case "$1" in
    "start")
        echo "启动所有pane监控..."
        echo "启动%17 pane监控..."
        ./manage_monitor_17.sh start
        echo ""
        echo "启动%19 pane监控..."
        ./manage_monitor_19.sh start
        echo ""
        echo "所有监控进程已启动"
        ;;
        
    "stop")
        echo "停止所有pane监控..."
        echo "停止%17 pane监控..."
        ./manage_monitor_17.sh stop
        echo ""
        echo "停止%19 pane监控..."
        ./manage_monitor_19.sh stop
        echo ""
        echo "所有监控进程已停止"
        ;;
        
    "restart")
        echo "重启所有pane监控..."
        echo "重启%17 pane监控..."
        ./manage_monitor_17.sh restart
        echo ""
        echo "重启%19 pane监控..."
        ./manage_monitor_19.sh restart
        echo ""
        echo "所有监控进程已重启"
        ;;
        
    "status")
        echo "=== 所有pane监控状态 ==="
        echo ""
        echo "📊 %17 pane监控状态:"
        ./manage_monitor_17.sh status
        echo ""
        echo "📊 %19 pane监控状态:"
        ./manage_monitor_19.sh status
        echo ""
        echo "=== PM2总览 ==="
        pm2 status | grep -E "(opencode-watch|online|stopped)"
        ;;
        
    "logs")
        echo "=== 所有pane监控实时日志 ==="
        echo "按 Ctrl+C 退出日志查看"
        echo ""
        echo "📄 %17 pane监控日志:"
        pm2 logs opencode-watch-17 &
        echo ""
        echo "📄 %19 pane监控日志:"
        pm2 logs opencode-watch-19 &
        wait
        ;;
        
    *)
        echo "使用方法: $0 [start|stop|restart|status|logs]"
        echo ""
        echo "命令说明:"
        echo "  start   - 启动所有监控进程"
        echo "  stop    - 停止所有监控进程"
        echo "  restart - 重启所有监控进程"
        echo "  status  - 查看所有进程状态"
        echo "  logs    - 查看所有实时日志"
        echo ""
        echo "快速状态查看:"
        echo "  %17 pane: ./manage_monitor_17.sh status"
        echo "  %19 pane: ./manage_monitor_19.sh status"
        exit 1
        ;;
esac