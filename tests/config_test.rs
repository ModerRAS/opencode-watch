use opencode_watch::config::Config;
use serde_yaml;

#[test]
fn test_default_config() {
    let config = Config::default();
    
    // 测试默认值
    assert_eq!(config.tmux.pane, "%18");
    assert_eq!(config.llm.backend, "ollama");
    assert_eq!(config.monitoring.interval, 5);
    assert_eq!(config.monitoring.stuck_sec, 30);
    assert_eq!(config.monitoring.max_retry, 3);
    assert_eq!(config.monitoring.command_delay_ms, 1000);
    assert_eq!(config.monitoring.enter_delay_ms, 3000);
    
    // 测试默认指令列表
    let expected_commands = vec![
        "继续".to_string(),
        "继续工作".to_string(),
        "请继续".to_string(),
        "/compact".to_string(),
    ];
    assert_eq!(config.monitoring.intervention_commands, expected_commands);
}

#[test]
fn test_config_serialization() {
    let config = Config::default();
    let yaml = serde_yaml::to_string(&config).unwrap();
    let deserialized: Config = serde_yaml::from_str(&yaml).unwrap();
    
    assert_eq!(config.tmux.pane, deserialized.tmux.pane);
    assert_eq!(config.llm.backend, deserialized.llm.backend);
    assert_eq!(config.monitoring.interval, deserialized.monitoring.interval);
}

#[test]
fn test_custom_intervention_commands() {
    let yaml = r#"
tmux:
  pane: "%19"
llm:
  backend: "none"
monitoring:
  interval: 10
  stuck_sec: 40
  max_retry: 5
  command_delay_ms: 1500
  enter_delay_ms: 3500
  intervention_commands:
    - "custom1"
    - "custom2"
    - "/custom3"
"#;
    
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    
    assert_eq!(config.tmux.pane, "%19");
    assert_eq!(config.monitoring.interval, 10);
    assert_eq!(config.monitoring.command_delay_ms, 1500);
    assert_eq!(config.monitoring.enter_delay_ms, 3500);
    
    let expected_commands = vec![
        "custom1".to_string(),
        "custom2".to_string(),
        "/custom3".to_string(),
    ];
    assert_eq!(config.monitoring.intervention_commands, expected_commands);
}

#[test]
fn test_get_next_intervention_command() {
    let mut config = Config::default();
    config.monitoring.intervention_commands = vec![
        "cmd1".to_string(),
        "cmd2".to_string(),
        "cmd3".to_string(),
    ];
    
    // 测试指令循环
    let (cmd, idx) = config.get_next_intervention_command(0);
    assert_eq!(cmd, "cmd2");
    assert_eq!(idx, 1);
    
    let (cmd, idx) = config.get_next_intervention_command(1);
    assert_eq!(cmd, "cmd3");
    assert_eq!(idx, 2);
    
    let (cmd, idx) = config.get_next_intervention_command(2);
    assert_eq!(cmd, "cmd1");
    assert_eq!(idx, 0);
    
    // 测试空列表情况
    config.monitoring.intervention_commands = vec![];
    let (cmd, idx) = config.get_next_intervention_command(0);
    assert_eq!(cmd, "继续");
    assert_eq!(idx, 0);
}

#[test]
fn test_config_with_missing_optional_fields() {
    let yaml = r#"
tmux:
  pane: "%17"
llm:
  backend: "ollama"
monitoring:
  interval: 8
  stuck_sec: 25
  max_retry: 4
"#;
    
    let config: Config = serde_yaml::from_str(yaml).unwrap();
    
    // 检查默认值
    assert_eq!(config.tmux.pane, "%17");
    assert_eq!(config.llm.backend, "ollama");
    assert_eq!(config.monitoring.interval, 8);
    assert_eq!(config.monitoring.stuck_sec, 25);
    assert_eq!(config.monitoring.max_retry, 4);
    assert_eq!(config.monitoring.command_delay_ms, 1000);
    assert_eq!(config.monitoring.enter_delay_ms, 3000);
    
    // 检查默认指令列表
    assert!(!config.monitoring.intervention_commands.is_empty());
    assert_eq!(config.monitoring.intervention_commands[0], "继续");
}