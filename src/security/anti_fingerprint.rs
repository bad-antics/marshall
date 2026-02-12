//! Anti-fingerprinting protections
//! Rotates user agents, spoofs headers, and normalises OS/hardware signals

use rand::Rng;

/// Common screen resolutions to randomise per session
const COMMON_RESOLUTIONS: &[(u32, u32)] = &[
    (1920, 1080), (2560, 1440), (1366, 768), (1536, 864),
    (1440, 900), (1680, 1050), (1280, 720), (1600, 900),
];

/// Hardware concurrency values typical of mid-range to high-end machines
const COMMON_CORE_COUNTS: &[u32] = &[4, 6, 8, 12, 16];

pub struct AntiFingerprintEngine {
    user_agent: String,
    accept_language: String,
    screen_resolution: (u32, u32),
    hardware_concurrency: u32,
    timezone_offset: i32,
}

impl AntiFingerprintEngine {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let res = COMMON_RESOLUTIONS[rng.gen_range(0..COMMON_RESOLUTIONS.len())];
        let cores = COMMON_CORE_COUNTS[rng.gen_range(0..COMMON_CORE_COUNTS.len())];

        Self {
            user_agent: Self::generate_user_agent(),
            accept_language: "en-US,en;q=0.9".into(),
            screen_resolution: res,
            hardware_concurrency: cores,
            timezone_offset: 0,
        }
    }
    
    fn generate_user_agent() -> String {
        // Rotate between common, high-entropy user agents across OS/browser combos
        let agents = [
            // Chrome – Windows
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
            // Chrome – macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
            // Chrome – Linux
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36",
            // Firefox – Windows
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:125.0) Gecko/20100101 Firefox/125.0",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:124.0) Gecko/20100101 Firefox/124.0",
            // Firefox – macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.15; rv:125.0) Gecko/20100101 Firefox/125.0",
            // Firefox – Linux
            "Mozilla/5.0 (X11; Linux x86_64; rv:125.0) Gecko/20100101 Firefox/125.0",
            // Edge – Windows
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0",
            // Safari – macOS
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 14_4) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4 Safari/605.1.15",
        ];
        
        let mut rng = rand::thread_rng();
        agents[rng.gen_range(0..agents.len())].into()
    }
    
    pub fn get_spoofed_headers(&self) -> Vec<(String, String)> {
        vec![
            ("User-Agent".into(), self.user_agent.clone()),
            ("Accept-Language".into(), self.accept_language.clone()),
            ("Accept-Encoding".into(), "gzip, deflate, br".into()),
            ("Accept".into(), "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8".into()),
            ("Sec-Fetch-Dest".into(), "document".into()),
            ("Sec-Fetch-Mode".into(), "navigate".into()),
            ("Sec-Fetch-Site".into(), "none".into()),
            ("Sec-Fetch-User".into(), "?1".into()),
            ("DNT".into(), "1".into()),
            ("Sec-GPC".into(), "1".into()),
        ]
    }

    /// Generate JavaScript to inject into pages that spoofs navigator properties
    pub fn generate_spoofing_script(&self) -> String {
        format!(
            r#"
            (function() {{
                Object.defineProperty(navigator, 'hardwareConcurrency', {{ get: () => {cores} }});
                Object.defineProperty(screen, 'width',  {{ get: () => {sw} }});
                Object.defineProperty(screen, 'height', {{ get: () => {sh} }});
                Object.defineProperty(screen, 'availWidth',  {{ get: () => {sw} }});
                Object.defineProperty(screen, 'availHeight', {{ get: () => {sh} }});
                Object.defineProperty(navigator, 'deviceMemory', {{ get: () => 8 }});
                Object.defineProperty(navigator, 'maxTouchPoints', {{ get: () => 0 }});
                const origGetTimezoneOffset = Date.prototype.getTimezoneOffset;
                Date.prototype.getTimezoneOffset = function() {{ return {tz}; }};
                const origToDataURL = HTMLCanvasElement.prototype.toDataURL;
                HTMLCanvasElement.prototype.toDataURL = function(type) {{
                    const ctx = this.getContext('2d');
                    if (ctx) {{
                        const imageData = ctx.getImageData(0, 0, this.width, this.height);
                        for (let i = 0; i < imageData.data.length; i += 4) {{
                            imageData.data[i] ^= 1;
                        }}
                        ctx.putImageData(imageData, 0, 0);
                    }}
                    return origToDataURL.apply(this, arguments);
                }};
            }})();
            "#,
            cores = self.hardware_concurrency,
            sw = self.screen_resolution.0,
            sh = self.screen_resolution.1,
            tz = self.timezone_offset,
        )
    }

    pub fn user_agent(&self) -> &str { &self.user_agent }
    pub fn screen_resolution(&self) -> (u32, u32) { self.screen_resolution }
}

impl Default for AntiFingerprintEngine {
    fn default() -> Self { Self::new() }
}
