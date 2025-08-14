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
    if let Some(backend) = &args.backend {
        config.llm.backend = backend.clone();
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
    println!("使用 LLM 后端: {}", config.llm.backend);
    println!("监控 tmux pane: {}", config.tmux.pane);
    println!("监控间隔: {} 秒", config.monitoring.interval);
    println!("卡住判定: {} 秒", config.monitoring.stuck_sec);
    println!("最大重试: {} 次", config.monitoring.max_retry);
    println!("按 Ctrl+C 退出");

    let tmux_client = TmuxClient::new();
    let mut last_content = String::new();
    let mut stuck_count = 0;

    // 创建Tokio运行时
    let rt = Runtime::new().unwrap();

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
                
                // 检查是否有变化
                if content == last_content {
                    stuck_count += 1;
                    println!("⏸️  内容无变化 (第{}次)", stuck_count);
                    
                    if stuck_count >= 3 {
                        println!("🚨 检测到可能卡住状态!");
                        
                        // 使用LLM分析
                        if config.llm.backend != "none" {
                            println!("🤖 使用LLM分析状态...");
                            let llm_client = llm::LlmClient::new(&config.llm.backend, "llama3.2");
                            match rt.block_on(llm_client.analyze_state(&content)) {
                                Ok(analysis) => {
                                    println!("🧠 LLM分析结果: {}", analysis);
                                    
                                    if analysis.contains("卡住") || analysis.contains("stuck") {
                                        println!("🔧 LLM确认卡住，发送继续指令");
                                        // 发送"继续"指令
                                        if let Err(e) = tmux_client.send_keys(&config.tmux.pane, "继续") {
                                            eprintln!("❌ 发送'继续'指令失败: {}", e);
                                        } else {
                                            println!("✅ 已发送'继续'指令");
                                            // 等待一下再发送回车
                                            thread::sleep(Duration::from_millis(500));
                                            if let Err(e) = tmux_client.send_keys(&config.tmux.pane, "Enter") {
                                                eprintln!("❌ 发送回车失败: {}", e);
                                            } else {
                                                println!("✅ 已发送回车，指令执行完成");
                                            }
                                        }
                                    } else {
                                        println!("✅ LLM认为状态正常，可能是暂时等待");
                                    }
                                }
                                Err(e) => {
                                    eprintln!("❌ LLM分析失败: {}", e);
                                }
                            }
                        } else {
                            println!("🔧 检测到卡住，发送继续指令 (无LLM分析)");
                            // 发送"继续"指令
                            if let Err(e) = tmux_client.send_keys(&config.tmux.pane, "继续") {
                                eprintln!("❌ 发送'继续'指令失败: {}", e);
                            } else {
                                println!("✅ 已发送'继续'指令");
                                // 等待一下再发送回车
                                thread::sleep(Duration::from_millis(500));
                                if let Err(e) = tmux_client.send_keys(&config.tmux.pane, "Enter") {
                                    eprintln!("❌ 发送回车失败: {}", e);
                                } else {
                                    println!("✅ 已发送回车，指令执行完成");
                                }
                            }
                        }
                        
                        // 重置计数器，继续监控
                        stuck_count = 0;
                    }
                } else {
                    stuck_count = 0;
                    println!("✅ 检测到内容变化");
                }
                
                last_content = content;
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