use std::time::{Duration, Instant};
use tokio::time::sleep;
use crate::config::Config;
use crate::tmux::TmuxClient;
use crate::activity::ActivityDetector;
use crate::llm::LlmClient;
use anyhow::Result;

pub async fn run_monitoring_loop(
    config: &Config,
    _last_active: &mut Instant,
    _retry_count: &mut usize,
) -> Result<()> {
    let tmux_client = TmuxClient::new();
    let mut activity_detector = ActivityDetector::new();
    let llm_client = LlmClient::new(&config.llm.backend, "llama3.2");
    let mut last_active_local = Instant::now();
    let mut retry_count_local = 0;
    
    loop {
        sleep(Duration::from_secs(config.monitoring.interval)).await;
        
        // 检查tmux pane是否存在
        if !tmux_client.check_pane_exists(&config.tmux.pane) {
            eprintln!("tmux pane {} 不存在", config.tmux.pane);
            continue;
        }
        
        // 检查活动
        match activity_detector.check_activity(&config.tmux.pane) {
            Ok(has_activity) => {
                if has_activity {
                    last_active_local = Instant::now();
                    retry_count_local = 0;
                    println!("检测到活动");
                } else {
                    let inactive_duration = last_active_local.elapsed();
                    if inactive_duration.as_secs() > config.monitoring.stuck_sec {
                        println!("检测到无活动 {} 秒，可能卡住", inactive_duration.as_secs());
                        
                        // 使用LLM分析状态
                        match llm_client.analyze_state(&tmux_client.capture_pane_content(&config.tmux.pane)?).await {
                            Ok(analysis) => {
                                println!("LLM分析结果: {}", analysis);
                                
                                // 根据分析结果决定是否需要干预
                                if analysis.contains("卡住") || analysis.contains("stuck") {
                                    retry_count_local += 1;
                                    if retry_count_local <= config.monitoring.max_retry {
                                        println!("尝试干预 (第{}次)", retry_count_local);
                                        tmux_client.send_keys(&config.tmux.pane, "C-c")?;
                                    } else {
                                        println!("达到最大重试次数，停止干预");
                                        break;
                                    }
                                } else {
                                    retry_count_local = 0;
                                }
                            }
                            Err(e) => {
                                eprintln!("LLM分析失败: {}", e);
                            }
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