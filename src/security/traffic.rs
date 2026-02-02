//! Traffic analysis protection

pub struct TrafficProtection {
    padding_enabled: bool,
    timing_jitter: bool,
}

impl TrafficProtection {
    pub fn new() -> Self {
        Self {
            padding_enabled: true,
            timing_jitter: true,
        }
    }
    
    /// Add random padding to requests to mask true size
    pub fn pad_request(&self, data: &[u8]) -> Vec<u8> {
        if !self.padding_enabled {
            return data.to_vec();
        }
        
        let mut rng = rand::thread_rng();
        let padding_size: usize = rand::Rng::gen_range(&mut rng, 64..512);
        
        let mut padded = data.to_vec();
        padded.extend(vec![0u8; padding_size]);
        
        padded
    }
    
    /// Add jitter to request timing
    pub async fn apply_timing_jitter(&self) {
        if !self.timing_jitter {
            return;
        }
        
        let mut rng = rand::thread_rng();
        let jitter_ms: u64 = rand::Rng::gen_range(&mut rng, 10..100);
        
        tokio::time::sleep(tokio::time::Duration::from_millis(jitter_ms)).await;
    }
}
