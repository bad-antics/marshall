// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Conversation management and command processing

use std::collections::HashMap;
use regex::Regex;
use chrono::{DateTime, Utc};
use super::{AssistantResponse, AssistantAction};

/// Conversation history entry
#[derive(Debug, Clone)]
pub struct ConversationEntry {
    pub user_input: String,
    pub assistant_response: String,
    pub timestamp: DateTime<Utc>,
    pub action_taken: Option<AssistantAction>,
}

/// Intent recognized from user input
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Intent {
    Greeting,
    Farewell,
    Help,
    Search,
    Navigate,
    Call,
    Workforce,
    TimeCard,
    Project,
    OSINT,
    Vulnerability,
    PortScan,
    Whois,
    Settings,
    Unknown,
}

/// Conversation manager handles command processing
pub struct ConversationManager {
    pub history: Vec<ConversationEntry>,
    pub context: HashMap<String, String>,
    intent_patterns: HashMap<Intent, Vec<Regex>>,
}

impl ConversationManager {
    pub fn new() -> Self {
        let mut manager = Self {
            history: Vec::new(),
            context: HashMap::new(),
            intent_patterns: HashMap::new(),
        };
        manager.initialize_patterns();
        manager
    }

    fn initialize_patterns(&mut self) {
        let patterns = vec![
            (Intent::Greeting, vec![
                r"(?i)^(hey|hi|hello|howdy|greetings)",
                r"(?i)good\s*(morning|afternoon|evening)",
            ]),
            (Intent::Farewell, vec![
                r"(?i)^(bye|goodbye|see you|later|exit|quit)",
            ]),
            (Intent::Help, vec![
                r"(?i)(help|assist|what can you do|commands)",
            ]),
            (Intent::Search, vec![
                r"(?i)(search|find|look up|lookup|google)\s+(.+)",
                r"(?i)search\s+for\s+(.+)",
            ]),
            (Intent::Navigate, vec![
                r"(?i)(go to|open|navigate|visit)\s+(.+)",
                r"(?i)take me to\s+(.+)",
            ]),
            (Intent::Call, vec![
                r"(?i)(call|dial|phone|ring)\s+(.+)",
                r"(?i)make a call to\s+(.+)",
            ]),
            (Intent::Workforce, vec![
                r"(?i)(workforce|employees|workers|team|staff)",
                r"(?i)show\s+(workforce|team|employees)",
            ]),
            (Intent::TimeCard, vec![
                r"(?i)(timecard|time card|clock|punch)\s*(in|out)?",
                r"(?i)(check|view|show)\s+time",
            ]),
            (Intent::Project, vec![
                r"(?i)(project|task|assignment)\s*(status|progress)?",
                r"(?i)(show|list|view)\s+projects?",
            ]),
            (Intent::OSINT, vec![
                r"(?i)(osint|recon|reconnaissance|investigate)\s+(.+)",
                r"(?i)gather\s+(intel|intelligence)\s+on\s+(.+)",
            ]),
            (Intent::Vulnerability, vec![
                r"(?i)(vuln|vulnerability|vulnerabilities|cve)\s*(scan)?\s*(.+)?",
                r"(?i)scan\s+for\s+(vulnerabilities|vulns)",
            ]),
            (Intent::PortScan, vec![
                r"(?i)(port|ports)\s*(scan)?\s*(.+)?",
                r"(?i)scan\s+ports?\s+(.+)",
            ]),
            (Intent::Whois, vec![
                r"(?i)whois\s+(.+)",
                r"(?i)(who is|domain info|lookup domain)\s+(.+)",
            ]),
            (Intent::Settings, vec![
                r"(?i)(settings|preferences|config|configure)",
            ]),
        ];

        for (intent, pattern_strs) in patterns {
            let regexes: Vec<Regex> = pattern_strs
                .iter()
                .filter_map(|p| Regex::new(p).ok())
                .collect();
            self.intent_patterns.insert(intent, regexes);
        }
    }

    pub fn process(&self, input: &str) -> AssistantResponse {
        let intent = self.classify_intent(input);
        let (response_text, action) = self.generate_response(&intent, input);
        
        AssistantResponse::new(&response_text, action)
    }

    fn classify_intent(&self, input: &str) -> Intent {
        for (intent, patterns) in &self.intent_patterns {
            for pattern in patterns {
                if pattern.is_match(input) {
                    return intent.clone();
                }
            }
        }
        Intent::Unknown
    }

    fn generate_response(&self, intent: &Intent, input: &str) -> (String, Option<AssistantAction>) {
        match intent {
            Intent::Greeting => (
                "Hello! Welcome to Marshall Command Center. I'm here to help with search, OSINT, workforce management, and calls. What would you like to do?".to_string(),
                None
            ),
            Intent::Farewell => (
                "Goodbye! Stay secure out there.".to_string(),
                None
            ),
            Intent::Help => (
                r#"I can help you with:
• **Search** - "Search for [topic]" - OSINT-enhanced search results
• **OSINT** - "Investigate [target]" - Full reconnaissance
• **WHOIS** - "Whois [domain]" - Domain information
• **Port Scan** - "Scan ports [target]" - Open ports detection
• **Vulnerability** - "Vuln scan [target]" - Security analysis
• **Workforce** - "Show workforce" - Employee management
• **Time Cards** - "Clock in/out" - Time tracking
• **Projects** - "Show projects" - Project monitoring
• **Call** - "Call [contact]" - VoIP calling

Just tell me what you need!"#.to_string(),
                None
            ),
            Intent::Search => {
                let query = self.extract_query(input, &["search", "find", "look up", "lookup"]);
                (
                    format!("Searching for '{}' with full OSINT enrichment...", query),
                    Some(AssistantAction::Search(query))
                )
            },
            Intent::Navigate => {
                let url = self.extract_query(input, &["go to", "open", "navigate", "visit", "take me to"]);
                (
                    format!("Navigating to {}...", url),
                    Some(AssistantAction::Navigate(url))
                )
            },
            Intent::Call => {
                let contact = self.extract_query(input, &["call", "dial", "phone", "ring"]);
                (
                    format!("Initiating call to {}...", contact),
                    Some(AssistantAction::Call(contact))
                )
            },
            Intent::Workforce => (
                "Opening Workforce Control Center...".to_string(),
                Some(AssistantAction::ShowWorkforce)
            ),
            Intent::TimeCard => (
                "Opening time card management...".to_string(),
                Some(AssistantAction::ShowWorkforce)
            ),
            Intent::Project => (
                "Loading project dashboard...".to_string(),
                Some(AssistantAction::ShowWorkforce)
            ),
            Intent::OSINT => {
                let target = self.extract_query(input, &["osint", "recon", "investigate", "intel on", "intelligence on"]);
                (
                    format!("Running full OSINT reconnaissance on '{}'...", target),
                    Some(AssistantAction::RunOSINT(target))
                )
            },
            Intent::Vulnerability => {
                let target = self.extract_query(input, &["vuln", "vulnerability", "vulnerabilities"]);
                (
                    format!("Scanning '{}' for vulnerabilities...", target),
                    Some(AssistantAction::RunOSINT(target))
                )
            },
            Intent::PortScan => {
                let target = self.extract_query(input, &["port", "ports", "scan ports"]);
                (
                    format!("Scanning open ports on '{}'...", target),
                    Some(AssistantAction::RunOSINT(target))
                )
            },
            Intent::Whois => {
                let domain = self.extract_query(input, &["whois", "who is", "domain info"]);
                (
                    format!("Looking up WHOIS information for '{}'...", domain),
                    Some(AssistantAction::RunOSINT(domain))
                )
            },
            Intent::Settings => (
                "Opening settings panel...".to_string(),
                Some(AssistantAction::Custom("settings".to_string()))
            ),
            Intent::Unknown => (
                format!("I'm not sure what you mean by '{}'. Try asking for 'help' to see what I can do.", input),
                None
            ),
        }
    }

    fn extract_query(&self, input: &str, keywords: &[&str]) -> String {
        let lower = input.to_lowercase();
        for keyword in keywords {
            if let Some(pos) = lower.find(keyword) {
                let after = &input[pos + keyword.len()..].trim();
                // Remove common words
                let cleaned = after
                    .trim_start_matches("for ")
                    .trim_start_matches("to ")
                    .trim_start_matches("on ")
                    .trim();
                if !cleaned.is_empty() {
                    return cleaned.to_string();
                }
            }
        }
        input.to_string()
    }

    pub fn add_to_history(&mut self, entry: ConversationEntry) {
        self.history.push(entry);
        // Keep only last 100 entries
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }

    pub fn clear_history(&mut self) {
        self.history.clear();
    }

    pub fn set_context(&mut self, key: &str, value: &str) {
        self.context.insert(key.to_string(), value.to_string());
    }

    pub fn get_context(&self, key: &str) -> Option<&String> {
        self.context.get(key)
    }
}

impl Default for ConversationManager {
    fn default() -> Self {
        Self::new()
    }
}
