use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "opencode-watch")]
#[command(about = "一个极简守护进程，基于opencode活动检测监控opencode")]
#[command(version = "1.0.0")]
pub struct Args {
    /// 配置文件路径
    #[arg(short, long, default_value = "config.yaml")]
    pub config: String,
    
    /// 要监控的 tmux pane ID
    #[arg(short, long)]
    pub pane: Option<String>,
    
    /// LLM 后端：ollama、openai、openrouter、none
    #[arg(short, long, value_parser = ["ollama", "openai", "openrouter", "none"])]
    pub backend: Option<String>,
    
    /// 检查间隔（秒）
    #[arg(short, long)]
    pub interval: Option<u64>,
    
    /// 无活动多久算卡住（秒）
    #[arg(short = 's', long)]
    pub stuck_sec: Option<u64>,
    
    /// 最大重试次数
    #[arg(short = 'm', long)]
    pub max_retry: Option<usize>,
}