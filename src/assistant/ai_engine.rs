// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Advanced AI Engine with multi-provider LLM support
//! Designed to match Grok-level conversational intelligence

use std::collections::HashMap;
use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use reqwest::Client;
use tokio::runtime::Runtime;

/// Supported AI providers
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Groq,
    XAI,          // Grok
    Ollama,       // Local
    LMStudio,     // Local
    OpenRouter,   // Aggregator
    Together,     // Together AI
    Mistral,
    Cohere,
    Custom,
}

impl AIProvider {
    pub fn endpoint(&self) -> &'static str {
        match self {
            Self::OpenAI => "https://api.openai.com/v1/chat/completions",
            Self::Anthropic => "https://api.anthropic.com/v1/messages",
            Self::Groq => "https://api.groq.com/openai/v1/chat/completions",
            Self::XAI => "https://api.x.ai/v1/chat/completions",
            Self::Ollama => "http://localhost:11434/api/chat",
            Self::LMStudio => "http://localhost:1234/v1/chat/completions",
            Self::OpenRouter => "https://openrouter.ai/api/v1/chat/completions",
            Self::Together => "https://api.together.xyz/v1/chat/completions",
            Self::Mistral => "https://api.mistral.ai/v1/chat/completions",
            Self::Cohere => "https://api.cohere.ai/v1/chat",
            Self::Custom => "",
        }
    }

    pub fn default_model(&self) -> &'static str {
        match self {
            Self::OpenAI => "gpt-4-turbo-preview",
            Self::Anthropic => "claude-3-opus-20240229",
            Self::Groq => "llama-3.1-70b-versatile",
            Self::XAI => "grok-2",
            Self::Ollama => "llama3.1:70b",
            Self::LMStudio => "local-model",
            Self::OpenRouter => "anthropic/claude-3-opus",
            Self::Together => "meta-llama/Llama-3.1-70B-Instruct-Turbo",
            Self::Mistral => "mistral-large-latest",
            Self::Cohere => "command-r-plus",
            Self::Custom => "custom",
        }
    }

    pub fn requires_auth(&self) -> bool {
        !matches!(self, Self::Ollama | Self::LMStudio)
    }
}

/// AI model configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModelConfig {
    pub provider: AIProvider,
    pub model: String,
    pub api_key: Option<String>,
    pub endpoint: Option<String>,
    pub temperature: f32,
    pub max_tokens: u32,
    pub top_p: f32,
    pub frequency_penalty: f32,
    pub presence_penalty: f32,
    pub stream: bool,
}

impl Default for AIModelConfig {
    fn default() -> Self {
        Self {
            provider: AIProvider::Ollama,
            model: "llama3.1:8b".to_string(),
            api_key: None,
            endpoint: None,
            temperature: 0.7,
            max_tokens: 4096,
            top_p: 0.95,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
            stream: false,
        }
    }
}

/// Message role in conversation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    System,
    User,
    Assistant,
    Tool,
}

/// Chat message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl ChatMessage {
    pub fn system(content: &str) -> Self {
        Self {
            role: MessageRole::System,
            content: content.to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn user(content: &str) -> Self {
        Self {
            role: MessageRole::User,
            content: content.to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }

    pub fn assistant(content: &str) -> Self {
        Self {
            role: MessageRole::Assistant,
            content: content.to_string(),
            name: None,
            tool_calls: None,
            tool_call_id: None,
        }
    }
}

/// Tool call for function calling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: String,
}

/// Tool/Function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    #[serde(rename = "type")]
    pub tool_type: String,
    pub function: FunctionDefinition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

/// Dr. Marshall's comprehensive system prompt
const DR_MARSHALL_SYSTEM_PROMPT: &str = r#"You are Dr. Marshall, an advanced AI assistant integrated into the Marshall Privacy Browser. You are sophisticated, knowledgeable, and designed to provide Grok-level intelligence with a focus on security, privacy, and productivity.

## Core Identity
- Name: Dr. Marshall
- Role: Chief Intelligence Officer for NullSec Security
- Personality: Professional yet approachable, highly technical but can explain simply, security-focused, witty when appropriate
- Style: Direct, efficient, insightful - like having a senior security researcher as your assistant

## Capabilities
You have access to the following integrated tools and can help with:

### OSINT & Security
- Domain/IP reconnaissance and WHOIS lookups
- Port scanning and service enumeration
- Vulnerability assessment and CVE research
- Threat intelligence and IOC analysis
- Social media and public records investigation
- Email verification and breach checking

### Workforce Management
- Employee time tracking and scheduling
- Project management and task assignment
- Payroll calculations and reporting
- Performance metrics and analytics
- Team communication coordination

### VoIP Communications
- SIP/WebRTC calling capabilities
- Contact management and call history
- Voicemail and call recording
- Conference calling setup

### Browser Assistance
- Privacy-enhanced web searching
- Bookmark and history management
- Ad/tracker blocking configuration
- Password and credential management
- Dark web monitoring alerts

## Response Guidelines

1. **Be Grok-like**: Provide insightful, nuanced responses. Don't just answer - analyze, contextualize, and offer additional relevant insights.

2. **Security-First**: Always consider security implications. Warn users about potential risks. Suggest safer alternatives when appropriate.

3. **Technical Depth**: You can go deep on technical topics. Use proper terminology but explain when needed.

4. **Actionable**: When users ask for help, provide specific, actionable steps. Include commands, configurations, or code when relevant.

5. **Context-Aware**: Remember conversation context. Reference previous discussions. Build on established knowledge.

6. **Proactive**: Anticipate follow-up questions. Offer related information the user might find valuable.

7. **Witty but Professional**: A touch of humor is welcome, but keep it professional. Security is serious business.

8. **Privacy-Conscious**: Never ask for or store sensitive information unnecessarily. Remind users about privacy best practices.

## Response Format
- Use markdown for formatting when it improves readability
- Use code blocks with language tags for commands/code
- Use bullet points for lists
- Use headers for long responses
- Keep responses concise but complete

## Example Interactions

User: "Check if example.com is vulnerable"
Good Response: "I'll run a comprehensive security assessment on example.com:

**Domain Intel:**
- Registrar: [info]
- Created: [date]
- Nameservers: [list]

**Open Ports:**
- 80 (HTTP) - Apache 2.4.52
- 443 (HTTPS) - TLS 1.3
- 22 (SSH) - OpenSSH 8.9

**Security Findings:**
⚠️ Missing security headers: X-Frame-Options, CSP
⚠️ SSH exposed to internet
✅ TLS configuration is strong
✅ No known CVEs for detected versions

**Recommendations:**
1. Add security headers to web server config
2. Restrict SSH access via firewall or VPN
3. Consider implementing a WAF

Want me to generate the specific configurations for these fixes?"

Remember: You are the intelligence backbone of Marshall Browser. Be brilliant, be helpful, be secure."#;

/// AI Engine for natural language processing
pub struct AIEngine {
    pub config: Arc<RwLock<AIModelConfig>>,
    pub messages: Arc<RwLock<Vec<ChatMessage>>>,
    pub tools: Vec<ToolDefinition>,
    client: Client,
    runtime: Runtime,
}

impl AIEngine {
    pub fn new(config: AIModelConfig) -> Self {
        let mut engine = Self {
            config: Arc::new(RwLock::new(config)),
            messages: Arc::new(RwLock::new(Vec::new())),
            tools: Vec::new(),
            client: Client::new(),
            runtime: Runtime::new().expect("Failed to create Tokio runtime"),
        };
        
        engine.initialize_system_prompt();
        engine.initialize_tools();
        engine
    }

    fn initialize_system_prompt(&mut self) {
        let mut messages = self.messages.write();
        messages.push(ChatMessage::system(DR_MARSHALL_SYSTEM_PROMPT));
    }

    fn initialize_tools(&mut self) {
        self.tools = vec![
            ToolDefinition {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "osint_scan".to_string(),
                    description: "Perform OSINT reconnaissance on a target domain, IP, or entity".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "target": {
                                "type": "string",
                                "description": "The target to investigate (domain, IP, email, username)"
                            },
                            "scan_type": {
                                "type": "string",
                                "enum": ["full", "domain", "ip", "email", "social", "ports"],
                                "description": "Type of scan to perform"
                            }
                        },
                        "required": ["target"]
                    }),
                },
            },
            ToolDefinition {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "web_search".to_string(),
                    description: "Search the web with privacy-enhanced results".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "Search query"
                            },
                            "engines": {
                                "type": "array",
                                "items": {"type": "string"},
                                "description": "Search engines to use"
                            }
                        },
                        "required": ["query"]
                    }),
                },
            },
            ToolDefinition {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "navigate".to_string(),
                    description: "Navigate the browser to a URL".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "url": {
                                "type": "string",
                                "description": "URL to navigate to"
                            }
                        },
                        "required": ["url"]
                    }),
                },
            },
            ToolDefinition {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "voip_call".to_string(),
                    description: "Initiate a VoIP call".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "contact": {
                                "type": "string",
                                "description": "Contact name or number to call"
                            }
                        },
                        "required": ["contact"]
                    }),
                },
            },
            ToolDefinition {
                tool_type: "function".to_string(),
                function: FunctionDefinition {
                    name: "workforce_action".to_string(),
                    description: "Perform workforce management action".to_string(),
                    parameters: serde_json::json!({
                        "type": "object",
                        "properties": {
                            "action": {
                                "type": "string",
                                "enum": ["clock_in", "clock_out", "view_timecard", "view_schedule", "view_projects", "assign_task"],
                                "description": "Workforce action to perform"
                            },
                            "employee_id": {
                                "type": "string",
                                "description": "Employee ID if applicable"
                            }
                        },
                        "required": ["action"]
                    }),
                },
            },
        ];
    }

    /// Send a message and get AI response
    pub fn chat(&self, user_input: &str) -> Result<String, AIError> {
        // Add user message
        {
            let mut messages = self.messages.write();
            messages.push(ChatMessage::user(user_input));
        }

        // Get response from AI
        let response = self.runtime.block_on(self.send_request())?;

        // Add assistant response to history
        {
            let mut messages = self.messages.write();
            messages.push(ChatMessage::assistant(&response));
        }

        Ok(response)
    }

    /// Send request to AI provider
    async fn send_request(&self) -> Result<String, AIError> {
        let config = self.config.read().clone();
        let messages = self.messages.read().clone();

        let endpoint = config.endpoint.as_deref()
            .unwrap_or_else(|| config.provider.endpoint());

        match config.provider {
            AIProvider::Anthropic => self.send_anthropic_request(&config, &messages, endpoint).await,
            AIProvider::Cohere => self.send_cohere_request(&config, &messages, endpoint).await,
            AIProvider::Ollama => self.send_ollama_request(&config, &messages, endpoint).await,
            _ => self.send_openai_compatible_request(&config, &messages, endpoint).await,
        }
    }

    /// OpenAI-compatible API request (works with OpenAI, Groq, XAI, Together, Mistral, OpenRouter, LMStudio)
    async fn send_openai_compatible_request(
        &self,
        config: &AIModelConfig,
        messages: &[ChatMessage],
        endpoint: &str,
    ) -> Result<String, AIError> {
        let mut request = serde_json::json!({
            "model": config.model,
            "messages": messages,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
            "top_p": config.top_p,
            "stream": config.stream,
        });

        // Add tools if supported
        if !self.tools.is_empty() && config.provider != AIProvider::LMStudio {
            request["tools"] = serde_json::to_value(&self.tools).unwrap_or_default();
        }

        let mut req = self.client.post(endpoint)
            .header("Content-Type", "application/json")
            .json(&request);

        if let Some(api_key) = &config.api_key {
            req = req.header("Authorization", format!("Bearer {}", api_key));
        }

        // Provider-specific headers
        if config.provider == AIProvider::OpenRouter {
            req = req.header("HTTP-Referer", "https://marshall.nullsec.io");
            req = req.header("X-Title", "Marshall Browser");
        }

        let response = req.send().await.map_err(|e| AIError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("HTTP {}: {}", status, text)));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|e| AIError::ParseError(e.to_string()))?;

        // Extract content from response
        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AIError::ParseError("No content in response".to_string()))
    }

    /// Anthropic Claude API request
    async fn send_anthropic_request(
        &self,
        config: &AIModelConfig,
        messages: &[ChatMessage],
        endpoint: &str,
    ) -> Result<String, AIError> {
        // Convert messages to Anthropic format
        let system = messages.iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| m.content.clone())
            .unwrap_or_default();

        let anthropic_messages: Vec<serde_json::Value> = messages.iter()
            .filter(|m| m.role != MessageRole::System)
            .map(|m| {
                serde_json::json!({
                    "role": match m.role {
                        MessageRole::User => "user",
                        MessageRole::Assistant => "assistant",
                        _ => "user",
                    },
                    "content": m.content
                })
            })
            .collect();

        let request = serde_json::json!({
            "model": config.model,
            "max_tokens": config.max_tokens,
            "system": system,
            "messages": anthropic_messages,
        });

        let api_key = config.api_key.as_ref()
            .ok_or_else(|| AIError::ConfigError("Anthropic API key required".to_string()))?;

        let response = self.client.post(endpoint)
            .header("Content-Type", "application/json")
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("HTTP {}: {}", status, text)));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|e| AIError::ParseError(e.to_string()))?;

        json["content"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AIError::ParseError("No content in response".to_string()))
    }

    /// Ollama local API request
    async fn send_ollama_request(
        &self,
        config: &AIModelConfig,
        messages: &[ChatMessage],
        endpoint: &str,
    ) -> Result<String, AIError> {
        let ollama_messages: Vec<serde_json::Value> = messages.iter()
            .map(|m| {
                serde_json::json!({
                    "role": match m.role {
                        MessageRole::System => "system",
                        MessageRole::User => "user",
                        MessageRole::Assistant => "assistant",
                        MessageRole::Tool => "tool",
                    },
                    "content": m.content
                })
            })
            .collect();

        let request = serde_json::json!({
            "model": config.model,
            "messages": ollama_messages,
            "stream": false,
            "options": {
                "temperature": config.temperature,
                "top_p": config.top_p,
                "num_predict": config.max_tokens,
            }
        });

        let response = self.client.post(endpoint)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("HTTP {}: {}", status, text)));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|e| AIError::ParseError(e.to_string()))?;

        json["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AIError::ParseError("No content in response".to_string()))
    }

    /// Cohere API request
    async fn send_cohere_request(
        &self,
        config: &AIModelConfig,
        messages: &[ChatMessage],
        endpoint: &str,
    ) -> Result<String, AIError> {
        let preamble = messages.iter()
            .find(|m| m.role == MessageRole::System)
            .map(|m| m.content.clone())
            .unwrap_or_default();

        let chat_history: Vec<serde_json::Value> = messages.iter()
            .filter(|m| m.role != MessageRole::System)
            .take(messages.len().saturating_sub(2))
            .map(|m| {
                serde_json::json!({
                    "role": if m.role == MessageRole::User { "USER" } else { "CHATBOT" },
                    "message": m.content
                })
            })
            .collect();

        let last_message = messages.iter()
            .filter(|m| m.role == MessageRole::User)
            .last()
            .map(|m| m.content.clone())
            .unwrap_or_default();

        let request = serde_json::json!({
            "model": config.model,
            "message": last_message,
            "preamble": preamble,
            "chat_history": chat_history,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
        });

        let api_key = config.api_key.as_ref()
            .ok_or_else(|| AIError::ConfigError("Cohere API key required".to_string()))?;

        let response = self.client.post(endpoint)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| AIError::NetworkError(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("HTTP {}: {}", status, text)));
        }

        let json: serde_json::Value = response.json().await
            .map_err(|e| AIError::ParseError(e.to_string()))?;

        json["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| AIError::ParseError("No content in response".to_string()))
    }

    /// Update AI configuration
    pub fn set_config(&self, config: AIModelConfig) {
        *self.config.write() = config;
    }

    /// Get current configuration
    pub fn get_config(&self) -> AIModelConfig {
        self.config.read().clone()
    }

    /// Clear conversation history (keeps system prompt)
    pub fn clear_history(&self) {
        let mut messages = self.messages.write();
        messages.retain(|m| m.role == MessageRole::System);
    }

    /// Get conversation history
    pub fn get_history(&self) -> Vec<ChatMessage> {
        self.messages.read().clone()
    }

    /// Add context to the conversation
    pub fn add_context(&self, context: &str) {
        let mut messages = self.messages.write();
        messages.push(ChatMessage::system(&format!("Additional context: {}", context)));
    }

    /// Check if local AI (Ollama) is available
    pub fn check_local_ai(&self) -> bool {
        self.runtime.block_on(async {
            self.client.get("http://localhost:11434/api/tags")
                .send()
                .await
                .map(|r| r.status().is_success())
                .unwrap_or(false)
        })
    }

    /// List available Ollama models
    pub fn list_ollama_models(&self) -> Vec<String> {
        self.runtime.block_on(async {
            let response = self.client.get("http://localhost:11434/api/tags")
                .send()
                .await;

            if let Ok(resp) = response {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    return json["models"]
                        .as_array()
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                }
            }
            Vec::new()
        })
    }
}

impl Default for AIEngine {
    fn default() -> Self {
        Self::new(AIModelConfig::default())
    }
}

/// AI Error types
#[derive(Debug, Clone)]
pub enum AIError {
    NetworkError(String),
    APIError(String),
    ParseError(String),
    ConfigError(String),
    RateLimited,
    InvalidModel,
    TokenLimitExceeded,
}

impl std::fmt::Display for AIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NetworkError(e) => write!(f, "Network error: {}", e),
            Self::APIError(e) => write!(f, "API error: {}", e),
            Self::ParseError(e) => write!(f, "Parse error: {}", e),
            Self::ConfigError(e) => write!(f, "Config error: {}", e),
            Self::RateLimited => write!(f, "Rate limited - please wait"),
            Self::InvalidModel => write!(f, "Invalid model specified"),
            Self::TokenLimitExceeded => write!(f, "Token limit exceeded"),
        }
    }
}

impl std::error::Error for AIError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ai_engine_creation() {
        let engine = AIEngine::default();
        assert!(!engine.messages.read().is_empty());
    }

    #[test]
    fn test_provider_endpoints() {
        assert!(AIProvider::OpenAI.endpoint().contains("openai.com"));
        assert!(AIProvider::XAI.endpoint().contains("x.ai"));
        assert!(AIProvider::Ollama.endpoint().contains("localhost"));
    }
}
