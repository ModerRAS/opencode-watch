use serde::{Deserialize, Serialize};
use ollama_rs::generation::completion::request::GenerationRequest;
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LlmResponse {
    pub response: String,
}

pub struct LlmClient {
    backend: String,
    model: String,
}

impl LlmClient {
    pub fn new(backend: &str, model: &str) -> Self {
        LlmClient {
            backend: backend.to_string(),
            model: model.to_string(),
        }
    }
    
    pub async fn analyze_state(&self, content: &str) -> Result<String> {
        match self.backend.as_str() {
            "ollama" => self.analyze_with_ollama(content).await,
            "openai" => self.analyze_with_openai(content).await,
            "openrouter" => self.analyze_with_openrouter(content).await,
            "none" => Ok("无LLM分析".to_string()),
            _ => Err(anyhow::anyhow!("不支持的LLM后端")),
        }
    }
    
    async fn analyze_with_ollama(&self, content: &str) -> Result<String> {
        let client = ollama_rs::Ollama::default();
        
        let prompt = format!(
            "分析以下tmux pane内容，判断opencode是否处于卡住状态：\n\n{}",
            content
        );
        
        let request = GenerationRequest::new(self.model.clone(), prompt);
        let response = client.generate(request).await?;
        
        Ok(response.response)
    }
    
    async fn analyze_with_openai(&self, content: &str) -> Result<String> {
        let client = reqwest::Client::new();
        
        let request = LlmRequest {
            model: self.model.clone(),
            prompt: format!(
                "分析以下tmux pane内容，判断opencode是否处于卡住状态：\n\n{}",
                content
            ),
            stream: false,
        };
        
        let response = client
            .post("https://api.openai.com/v1/chat/completions")
            .json(&request)
            .send()
            .await?;
        
        let llm_response: LlmResponse = response.json().await?;
        Ok(llm_response.response)
    }
    
    async fn analyze_with_openrouter(&self, content: &str) -> Result<String> {
        let client = reqwest::Client::new();
        
        let request = LlmRequest {
            model: self.model.clone(),
            prompt: format!(
                "分析以下tmux pane内容，判断opencode是否处于卡住状态：\n\n{}",
                content
            ),
            stream: false,
        };
        
        let response = client
            .post("https://openrouter.ai/api/v1/chat/completions")
            .json(&request)
            .send()
            .await?;
        
        let llm_response: LlmResponse = response.json().await?;
        Ok(llm_response.response)
    }
}