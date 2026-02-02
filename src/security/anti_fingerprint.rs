//! Anti-fingerprinting protections

use rand::Rng;

pub struct AntiFingerprintEngine {
    user_agent: String,
    accept_language: String,
    screen_resolution: (u32, u32),
}

impl AntiFingerprintEngine {
    pub fn new() -> Self {
        Self {
            user_agent: Self::generate_user_agent(),
            accept_language: "en-US,en;q=0.9".into(),
            screen_resolution: (1920, 1080),
        }
    }
    
    fn generate_user_agent() -> String {
        // Rotate between common user agents
        let agents = [
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:122.0) Gecko/20100101 Firefox/122.0",
        ];
        
        let mut rng = rand::thread_rng();
        agents[rng.gen_range(0..agents.len())].into()
    }
    
    pub fn get_spoofed_headers(&self) -> Vec<(String, String)> {
        vec![
            ("User-Agent".into(), self.user_agent.clone()),
            ("Accept-Language".into(), self.accept_language.clone()),
            ("Accept-Encoding".into(), "gzip, deflate, br".into()),
        ]
    }
}
