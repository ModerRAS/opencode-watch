use dotenvy::dotenv;
use std::io::{self, Write};
use std::thread;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

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
use app::WatchApp;
use clap::Parser;

fn main() -> io::Result<()> {
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

    // 创建监控应用
    let mut app = WatchApp::new();
    
    // 启用原始模式以捕获键盘输入
    enable_raw_mode()?;
    
    println!("Opencode-Watch 启动成功");
    println!("使用 LLM 后端: {}", config.llm.backend);
    println!("监控 tmux pane: {}", config.tmux.pane);
    println!("按 Ctrl+C 退出");
    
    // 主循环：处理键盘输入和状态显示
    loop {
        // 显示当前状态
        print!("\r{}", app.get_display_string());
        io::stdout().flush()?;
        
        // 检查键盘输入
        if event::poll(std::time::Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(KeyEvent { code, modifiers, .. }) => {
                    match code {
                        KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                            println!("\n退出 Opencode-Watch");
                            break;
                        },
                        KeyCode::Enter => {
                            app.handle_input("\n");
                        },
                        KeyCode::Esc => {
                            app.handle_input("esc");
                        },
                        _ => {
                            // 其他键可以用于中断状态的恢复
                            if app.is_interrupted() {
                                app.handle_input("any");
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        
        // 如果在运行状态，执行监控逻辑
        if app.is_running() {
            // 这里可以集成原有的监控逻辑
            // 暂时简化处理
            thread::sleep(std::time::Duration::from_millis(100));
        }
    }
    
    // 禁用原始模式
    disable_raw_mode()?;
    
    Ok(())
}