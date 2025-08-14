use std::time::{Duration, Instant};
use tokio::time::sleep;
use crate::config::Config;
use crate::tmux::TmuxClient;
use crate::activity::ActivityDetector;
use anyhow::Result;

pub async fn run_monitoring_loop(
    config: &Config,
    _last_active: &mut Instant,
    _retry_count: &mut usize,
) -> Result<()> {
    let tmux_client = TmuxClient::new();
    let mut activity_detector = ActivityDetector::new();
    let mut last_active_local = Instant::now();
    let mut retry_count_local = 0;
    
    loop {
        sleep(Duration::from_secs(config.monitoring.interval)).await;
        
        // 检查tmux pane是否存在
        if !tmux_client.check_pane_exists(&config.tmux.pane) {
            eprintln!("tmux pane {} 不存在", config.tmux.pane);
            continue;
        }
        
        // 检查活动状态
        match activity_detector.check_activity(&config.tmux.pane) {
            Ok(is_active) => {
                if is_active {
                    // 检测到working或esc interrupt标识，正在工作
                    last_active_local = Instant::now();
                    retry_count_local = 0;
                    println!("检测到工作状态 (working/esc interrupt)");
                } else {
                    // 没有检测到working或esc interrupt标识，可能卡住
                    let inactive_duration = last_active_local.elapsed();
                    if inactive_duration.as_secs() > config.monitoring.stuck_sec {
                        println!("检测到工作标识消失 {} 秒，确认卡住", inactive_duration.as_secs());
                        
                        // 直接干预，不需要LLM分析，因为逻辑很明确：
                        // working和esc interrupt都消失 = 卡住
                        retry_count_local += 1;
                        if retry_count_local <= config.monitoring.max_retry {
                            println!("尝试干预 (第{}次)", retry_count_local);
                            
                            // 发送当前配置的命令列表中的下一个命令
                            let command_index = (retry_count_local - 1) % config.intervention.commands.len();
                            let command = &config.intervention.commands[command_index];
                            println!("发送命令: {}", command);
                            
                            tmux_client.send_keys(&config.tmux.pane, command)?;
                            
                            // 如果配置了回车键延迟，等待后发送回车
                            if config.intervention.enter_delay_ms > 0 {
                                sleep(Duration::from_millis(config.intervention.enter_delay_ms as u64)).await;
                                tmux_client.send_keys(&config.tmux.pane, "Enter")?;
                            }
                        } else {
                            println!("达到最大重试次数，停止干预");
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("活动检测失败: {}", e);
            }
        }
    }
    
    Ok(())
}