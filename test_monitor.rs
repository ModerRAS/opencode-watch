use std::time::Duration;
use std::thread;
use crate::tmux::TmuxClient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmux_client = TmuxClient::new();
    let pane = "%17";
    
    println!("开始监控 tmux pane: {}", pane);
    
    for i in 1..=5 {
        println!("--- 第{}次检查 ---", i);
        
        // 检查pane是否存在
        if !tmux_client.check_pane_exists(pane) {
            println!("tmux pane {} 不存在", pane);
            break;
        }
        
        // 捕获内容
        match tmux_client.capture_pane_content(pane) {
            Ok(content) => {
                let lines: Vec<&str> = content.lines().collect();
                println!("捕获到 {} 行内容", lines.len());
                
                // 显示最后几行
                let last_lines = lines.iter().rev().take(5).collect::<Vec<_>>();
                println!("最后5行内容:");
                for line in last_lines.iter().rev() {
                    println!("  {}", line);
                }
                
                // 检查是否有"Generating..."等卡住特征
                let has_generating = content.contains("Generating...");
                let has_build = content.contains("Build");
                
                println!("检测到 'Generating...': {}", has_generating);
                println!("检测到 'Build': {}", has_build);
                
                if has_generating {
                    println!("⚠️  可能卡住: 检测到 'Generating...'");
                }
            }
            Err(e) => {
                println!("捕获内容失败: {}", e);
            }
        }
        
        thread::sleep(Duration::from_secs(2));
    }
    
    println!("测试完成");
    Ok(())
}