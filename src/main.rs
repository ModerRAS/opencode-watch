use dotenvy::dotenv;
use std::time::Duration;
use std::thread;
use anyhow::Result;
use tokio::runtime::Runtime;

// 模块声明
mod config;
mod args;
mod tmux;
mod activity;
mod llm;
mod monitor;
mod state;
mod animation;
mod app;

// 使用声明
use config::Config;
use args::Args;
use tmux::TmuxClient;
use clap::Parser;

fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    // 加载配置
    let mut config = Config::load(&args.config).unwrap_or_else(|_| {
        eprintln!("无法加载配置文件 {}，使用默认配置", args.config);
        Config::default()
    });

    // 使用命令行参数覆盖配置（如果提供）
    if let Some(pane) = &args.pane {
        config.tmux.pane = pane.clone();
    }
    if let Some(interval) = args.interval {
        config.monitoring.interval = interval;
    }
    if let Some(stuck_sec) = args.stuck_sec {
        config.monitoring.stuck_sec = stuck_sec;
    }
    if let Some(max_retry) = args.max_retry {
        config.monitoring.max_retry = max_retry;
    }

    println!("Opencode-Watch 启动成功");
    println!("监控 tmux pane: {}", config.tmux.pane);
    println!("监控间隔: {} 秒", config.monitoring.interval);
    println!("卡住判定: {} 秒", config.monitoring.stuck_sec);
    println!("最大重试: {} 次", config.monitoring.max_retry);
    println!("干预指令数量: {} 个", config.intervention.commands.len());
    println!("按 Ctrl+C 退出");

    let tmux_client = TmuxClient::new();
    let mut stuck_count = 0;
    let mut command_index = 0;
    
    // 加载全局配置
    if let Err(e) = config::Config::load_global(&args.config) {
        eprintln!("警告: 无法加载全局配置: {}", e);
    }

    // 创建Tokio运行时（虽然不再需要LLM，但保留以备将来使用）
    let _rt = Runtime::new().unwrap();

    // 简单监控循环
    loop {
        println!("\n=== 检查时间: {:?} ===", std::time::SystemTime::now());
        
        // 检查pane是否存在
        if !tmux_client.check_pane_exists(&config.tmux.pane) {
            eprintln!("❌ tmux pane {} 不存在", config.tmux.pane);
            break;
        }

        // 捕获内容
        match tmux_client.capture_pane_content(&config.tmux.pane) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                println!("📄 捕获到 {} 行内容", lines.len());
                
                // 显示最后几行
                let last_lines = lines.iter().rev().take(3).collect::<Vec<_>>();
                println!("📝 最后3行内容:");
                for line in last_lines.iter().rev() {
                    println!("   {}", line);
                }
                
                // 检查opencode工作状态 - 基于working和esc interrupt标识
                let has_working = content.contains("working");
                let has_esc_interrupt = content.contains("esc interrupt");
                let is_active = has_working || has_esc_interrupt;
                
                if is_active {
                    // 检测到working或esc interrupt，正在工作
                    stuck_count = 0;
                    println!("✅ 检测到工作状态 (working: {}, esc interrupt: {})", has_working, has_esc_interrupt);
                } else {
                    // 没有检测到working或esc interrupt，可能卡住
                    stuck_count += 1;
                    println!("⏸️  工作标识消失 (第{}次)", stuck_count);
                    
                    if stuck_count >= 3 {
                        println!("🚨 检测到卡住状态!");
                        
                        // 获取下一个干预指令
                        let (command, new_index) = config.get_next_intervention_command(command_index);
                        command_index = new_index;
                        
                        println!("🔧 尝试干预指令 [{}/{}]: '{}'", command_index + 1, config.intervention.commands.len(), command);
                        
                        // 直接执行干预（基于明确的working/esc interrupt逻辑，不需要LLM分析）
                        if let Err(e) = tmux_client.send_keys(&config.tmux.pane, &command) {
                            eprintln!("❌ 发送指令失败: {}", e);
                        } else {
                            println!("✅ 已发送指令: '{}'", command);
                            
                            // 等待指令输入完成，然后发送回车
                            let command_delay = Duration::from_millis(config.intervention.command_delay_ms);
                            let enter_delay = Duration::from_millis(config.intervention.enter_delay_ms);
                            
                            // 特殊处理C-c命令（不需要回车）
                            if command == "C-c" {
                                println!("✅ Ctrl+C已发送，无需回车");
                            } else if command.starts_with('/') {
                                println!("✅ 命令指令已发送，无需回车");
                            } else {
                                println!("⏳ 等待 {}ms 后发送回车...", config.intervention.enter_delay_ms);
                                thread::sleep(command_delay);
                                
                                if let Err(e) = tmux_client.send_keys(&config.tmux.pane, "Enter") {
                                    eprintln!("❌ 发送回车失败: {}", e);
                                } else {
                                    thread::sleep(enter_delay - command_delay);
                                    println!("✅ 已发送回车，指令执行完成");
                                }
                            }
                        }
                        
                        // 重置计数器，继续监控
                        stuck_count = 0;
                    }
                }
            }
            Err(e) => {
                eprintln!("❌ 捕获内容失败: {}", e);
            }
        }
        
        // 等待下次检查
        thread::sleep(Duration::from_secs(config.monitoring.interval));
    }
    
    Ok(())
}