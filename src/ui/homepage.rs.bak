// Copyright (c) 2026 bad-antics
// Licensed under the MIT License. See LICENSE file in the project root.
// https://github.com/bad-antics/marshall

//! Marshall Browser - Custom Homepage and Internal Pages
//! Fully custom privacy-focused browser experience

/// Generate the main Marshall homepage
pub fn generate_homepage() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Marshall - Private Browser</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, BlinkMacSystemFont, sans-serif;
            min-height: 100vh;
        }
        body {
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            min-height: 100vh;
            padding: 2rem;
        }
        a { color: var(--red); text-decoration: none; }
        a:hover { text-decoration: underline; }
        .logo svg {
            width: 80px;
            height: 80px;
            margin-bottom: 2rem;
        }
        .brand {
            font-size: 48px;
            font-weight: 700;
            letter-spacing: 8px;
            color: var(--red);
            margin-bottom: 0.5rem;
        }
        .tagline {
            color: var(--fg-dim);
            font-size: 14px;
            letter-spacing: 2px;
            margin-bottom: 3rem;
        }
        .search-box {
            width: 100%;
            max-width: 600px;
            margin-bottom: 2rem;
        }
        .search-form {
            display: flex;
            gap: 12px;
        }
        .search-input {
            flex: 1;
            padding: 16px 20px;
            background: var(--bg2);
            border: 2px solid var(--border);
            border-radius: 12px;
            color: var(--fg);
            font-size: 16px;
            outline: none;
        }
        .search-input:focus {
            border-color: var(--red);
        }
        .search-btn {
            padding: 16px 32px;
            background: var(--red);
            border: none;
            border-radius: 12px;
            color: white;
            font-size: 16px;
            font-weight: 600;
            cursor: pointer;
        }
        .search-btn:hover {
            background: #cc0033;
        }
        .quick-links {
            display: flex;
            gap: 1rem;
            flex-wrap: wrap;
            justify-content: center;
            margin-bottom: 3rem;
        }
        .quick-link {
            padding: 10px 20px;
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 20px;
            color: var(--fg);
            font-size: 13px;
            text-decoration: none;
            transition: all 0.2s ease;
        }
        .quick-link:hover {
            border-color: var(--red);
            color: var(--red);
            text-decoration: none;
        }
        .features {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1.5rem;
            width: 100%;
            max-width: 800px;
        }
        .feature {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.5rem;
            text-align: center;
        }
        .feature-icon { font-size: 32px; margin-bottom: 1rem; }
        .feature-title { color: var(--red); font-weight: 600; margin-bottom: 0.5rem; }
        .feature-desc { color: var(--fg-dim); font-size: 13px; }
        .footer {
            margin-top: 4rem;
            color: var(--fg-dim);
            font-size: 12px;
        }
    </style>
</head>
<body>
    <div class="logo">
        <svg viewBox="0 0 80 80">
            <circle cx="40" cy="40" r="36" fill="none" stroke="#ff0040" stroke-width="3"/>
            <path d="M24 56V28l16 12 16-12v28" fill="none" stroke="#ff0040" stroke-width="4" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
    </div>
    <div class="brand">MARSHALL</div>
    <div class="tagline">NULLSEC PRIVATE BROWSER</div>
    <div class="search-box">
        <form class="search-form" action="https://duckduckgo.com/" method="GET">
            <input type="text" name="q" class="search-input" placeholder="Search privately..." autofocus>
            <input type="hidden" name="kae" value="d">
            <input type="hidden" name="k1" value="-1">
            <input type="hidden" name="kp" value="-2">
            <button type="submit" class="search-btn">Search</button>
        </form>
    </div>
    <div class="quick-links">
        <a href="javascript:void(0)" onclick="window.location.href='marshall://assistant'" class="quick-link">Dr Marshall AI</a>
        <a href="javascript:void(0)" onclick="window.location.href='marshall://osint'" class="quick-link">OSINT Tools</a>
        <a href="javascript:void(0)" onclick="window.location.href='marshall://settings'" class="quick-link">Settings</a>
        <a href="javascript:void(0)" onclick="window.location.href='marshall://privacy'" class="quick-link">Privacy</a>
    </div>
    <div class="features">
        <div class="feature">
            <div class="feature-icon">&#128274;</div>
            <div class="feature-title">Private Browsing</div>
            <div class="feature-desc">No tracking, no history, no traces</div>
        </div>
        <div class="feature">
            <div class="feature-icon">&#129302;</div>
            <div class="feature-title">Dr Marshall AI</div>
            <div class="feature-desc">Your private AI assistant</div>
        </div>
        <div class="feature">
            <div class="feature-icon">&#128269;</div>
            <div class="feature-title">OSINT Tools</div>
            <div class="feature-desc">Domain intel, port scans, WHOIS</div>
        </div>
        <div class="feature">
            <div class="feature-icon">&#128737;</div>
            <div class="feature-title">Ad Blocking</div>
            <div class="feature-desc">Built-in tracker protection</div>
        </div>
    </div>
    <div class="footer">
        Marshall v2.0.0 | NullSec Division | <span style="color: #00ff88;">&#9679;</span> Secure
    </div>
</body>
</html>"##.to_string()
}

/// Generate Dr Marshall AI Assistant page
pub fn generate_assistant_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Dr Marshall - AI Assistant</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --yellow: #ffcc00;
            --blue: #00a8ff;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            height: 100vh;
        }
        body { display: flex; flex-direction: column; }
        a { color: var(--red); text-decoration: none; }
        a:hover { text-decoration: underline; }
        code {
            background: var(--bg3);
            padding: 2px 6px;
            border-radius: 4px;
            font-family: 'Consolas', 'Monaco', monospace;
            font-size: 13px;
            color: var(--green);
        }
        pre {
            background: var(--bg3);
            padding: 12px;
            border-radius: 8px;
            overflow-x: auto;
            margin: 10px 0;
            border: 1px solid var(--border);
        }
        pre code {
            background: none;
            padding: 0;
        }
        .header {
            background: linear-gradient(180deg, var(--bg2) 0%, var(--bg) 100%);
            border-bottom: 2px solid var(--red);
            padding: 1rem 2rem;
            display: flex;
            align-items: center;
            gap: 1rem;
        }
        .header-avatar {
            width: 64px;
            height: 64px;
            display: flex;
            align-items: center;
            justify-content: center;
            background: radial-gradient(circle, rgba(255,0,64,0.2) 0%, transparent 70%);
            border-radius: 50%;
        }
        .plague-doctor {
            width: 56px;
            height: 56px;
            filter: drop-shadow(0 0 10px rgba(255,0,64,0.5));
        }
        .header-info h1 { font-size: 22px; color: var(--red); font-weight: 700; }
        .header-info p { font-size: 12px; color: var(--fg-dim); margin-top: 2px; }
        .header-actions {
            margin-left: auto;
            display: flex;
            gap: 12px;
        }
        .header-btn {
            padding: 8px 16px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 12px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
        }
        .header-btn:hover { border-color: var(--red); color: var(--red); }
        .status-indicator {
            display: flex;
            align-items: center;
            gap: 6px;
            font-size: 12px;
            color: var(--green);
        }
        .status-dot {
            width: 8px;
            height: 8px;
            background: var(--green);
            border-radius: 50%;
            animation: pulse 2s infinite;
        }
        @keyframes pulse {
            0%, 100% { opacity: 1; box-shadow: 0 0 0 0 rgba(0,255,136,0.4); }
            50% { opacity: 0.8; box-shadow: 0 0 0 8px rgba(0,255,136,0); }
        }
        .chat-container {
            flex: 1;
            overflow-y: auto;
            padding: 1.5rem 2rem;
            scroll-behavior: smooth;
        }
        .message {
            max-width: 85%;
            margin-bottom: 1.25rem;
            animation: fadeIn 0.3s ease;
        }
        @keyframes fadeIn {
            from { opacity: 0; transform: translateY(10px); }
            to { opacity: 1; transform: translateY(0); }
        }
        .message-assistant { margin-right: auto; }
        .message-user { margin-left: auto; }
        .message-content {
            padding: 1rem 1.25rem;
            border-radius: 12px;
            line-height: 1.7;
            font-size: 14px;
        }
        .message-assistant .message-content {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-left: 3px solid var(--red);
        }
        .message-user .message-content {
            background: linear-gradient(135deg, var(--red) 0%, #cc0033 100%);
            color: white;
        }
        .message-meta {
            font-size: 10px;
            color: var(--fg-dim);
            margin-top: 6px;
            padding: 0 4px;
        }
        .message-assistant .message-meta { text-align: left; }
        .message-user .message-meta { text-align: right; }
        .typing-indicator {
            display: flex;
            gap: 4px;
            padding: 1rem 1.25rem;
            background: var(--bg2);
            border: 1px solid var(--border);
            border-left: 3px solid var(--red);
            border-radius: 12px;
            width: fit-content;
        }
        .typing-dot {
            width: 8px;
            height: 8px;
            background: var(--fg-dim);
            border-radius: 50%;
            animation: typingBounce 1.4s infinite;
        }
        .typing-dot:nth-child(2) { animation-delay: 0.2s; }
        .typing-dot:nth-child(3) { animation-delay: 0.4s; }
        @keyframes typingBounce {
            0%, 60%, 100% { transform: translateY(0); }
            30% { transform: translateY(-8px); }
        }
        .suggestions {
            display: flex;
            flex-wrap: wrap;
            gap: 8px;
            margin-top: 1rem;
        }
        .suggestion {
            padding: 8px 14px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 20px;
            font-size: 12px;
            cursor: pointer;
            transition: all 0.2s;
        }
        .suggestion:hover {
            border-color: var(--red);
            color: var(--red);
            transform: translateY(-1px);
        }
        .info-box {
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 12px;
            margin: 10px 0;
        }
        .info-box.warning { border-left: 3px solid var(--yellow); }
        .info-box.success { border-left: 3px solid var(--green); }
        .info-box.info { border-left: 3px solid var(--blue); }
        .tag {
            display: inline-block;
            padding: 2px 8px;
            background: var(--bg3);
            border-radius: 4px;
            font-size: 11px;
            margin: 2px;
        }
        .tag.high { background: rgba(255,0,64,0.2); color: var(--red); }
        .tag.medium { background: rgba(255,204,0,0.2); color: var(--yellow); }
        .tag.low { background: rgba(0,255,136,0.2); color: var(--green); }
        .input-area {
            background: var(--bg2);
            border-top: 1px solid var(--border);
            padding: 1.25rem 2rem;
        }
        .input-form {
            display: flex;
            gap: 12px;
            max-width: 1000px;
            margin: 0 auto;
        }
        .chat-input {
            flex: 1;
            padding: 14px 18px;
            background: var(--bg3);
            border: 2px solid var(--border);
            border-radius: 12px;
            color: var(--fg);
            font-size: 14px;
            outline: none;
            transition: border-color 0.2s;
        }
        .chat-input:focus { border-color: var(--red); }
        .chat-input::placeholder { color: var(--fg-dim); }
        .send-btn {
            padding: 14px 28px;
            background: linear-gradient(135deg, var(--red) 0%, #cc0033 100%);
            border: none;
            border-radius: 12px;
            color: white;
            font-size: 14px;
            font-weight: 600;
            cursor: pointer;
            transition: transform 0.2s, box-shadow 0.2s;
        }
        .send-btn:hover {
            transform: translateY(-1px);
            box-shadow: 0 4px 12px rgba(255,0,64,0.3);
        }
        .quick-actions {
            display: flex;
            gap: 8px;
            margin-top: 12px;
            flex-wrap: wrap;
            justify-content: center;
        }
        .quick-btn {
            padding: 8px 14px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 11px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 6px;
            transition: all 0.2s;
        }
        .quick-btn:hover {
            border-color: var(--red);
            color: var(--red);
        }
        .quick-btn .icon { font-size: 14px; }
        ul, ol { margin: 8px 0 8px 20px; }
        li { margin: 4px 0; }
        /* Modal Styles */
        .modal-overlay {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background: rgba(0,0,0,0.8);
            display: flex;
            align-items: center;
            justify-content: center;
            z-index: 1000;
            backdrop-filter: blur(4px);
        }
        .modal {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 16px;
            width: 90%;
            max-width: 550px;
            max-height: 90vh;
            overflow: hidden;
            display: flex;
            flex-direction: column;
            box-shadow: 0 20px 60px rgba(0,0,0,0.5);
        }
        .modal-header {
            padding: 1.25rem 1.5rem;
            background: var(--bg3);
            border-bottom: 1px solid var(--border);
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .modal-header h2 {
            font-size: 18px;
            color: var(--red);
        }
        .modal-close {
            width: 32px;
            height: 32px;
            background: transparent;
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 20px;
            cursor: pointer;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .modal-close:hover {
            border-color: var(--red);
            color: var(--red);
        }
        .modal-body {
            padding: 1.5rem;
            overflow-y: auto;
            flex: 1;
        }
        .modal-footer {
            padding: 1rem 1.5rem;
            background: var(--bg3);
            border-top: 1px solid var(--border);
            display: flex;
            gap: 12px;
            justify-content: flex-end;
        }
        .settings-group {
            margin-bottom: 1.25rem;
        }
        .settings-group label {
            display: block;
            font-size: 13px;
            font-weight: 600;
            color: var(--fg);
            margin-bottom: 8px;
        }
        .settings-group select,
        .settings-group input[type="text"],
        .settings-group input[type="password"],
        .settings-group textarea {
            width: 100%;
            padding: 12px 14px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 14px;
            outline: none;
        }
        .settings-group select:focus,
        .settings-group input:focus,
        .settings-group textarea:focus {
            border-color: var(--red);
        }
        .settings-group small {
            display: block;
            font-size: 11px;
            color: var(--fg-dim);
            margin-top: 6px;
        }
        .settings-group input[type="range"] {
            -webkit-appearance: none;
            height: 6px;
            background: var(--bg3);
            border-radius: 3px;
            border: none;
        }
        .settings-group input[type="range"]::-webkit-slider-thumb {
            -webkit-appearance: none;
            width: 18px;
            height: 18px;
            background: var(--red);
            border-radius: 50%;
            cursor: pointer;
        }
        .modal-btn {
            padding: 10px 20px;
            border-radius: 8px;
            font-size: 13px;
            font-weight: 600;
            cursor: pointer;
            border: none;
        }
        .modal-btn.primary {
            background: var(--red);
            color: white;
        }
        .modal-btn.primary:hover {
            background: #cc0033;
        }
        .modal-btn.secondary {
            background: var(--bg3);
            border: 1px solid var(--border);
            color: var(--fg);
        }
        .modal-btn.secondary:hover {
            border-color: var(--green);
            color: var(--green);
        }
        .status-connected .status-dot { background: var(--green); }
        .status-connected #statusText { color: var(--green); }
        .status-error .status-dot { background: var(--red); }
        .status-error #statusText { color: var(--red); }
        .status-local .status-dot { background: var(--yellow); }
        .status-local #statusText { color: var(--yellow); }
    </style>
</head>
<body>
    <div class="header">
        <div class="header-avatar">
            <svg class="plague-doctor" viewBox="0 0 100 100" fill="none">
                <ellipse cx="50" cy="75" rx="35" ry="25" fill="#1a1a1a"/>
                <circle cx="50" cy="40" r="28" fill="#0d0d0d" stroke="#ff0040" stroke-width="2"/>
                <path d="M50 38 L78 52 L50 58 Z" fill="#ff0040"/>
                <path d="M50 38 L22 52 L50 58 Z" fill="#cc0033"/>
                <circle cx="38" cy="35" r="7" fill="#00ff88">
                    <animate attributeName="opacity" values="1;0.4;1" dur="3s" repeatCount="indefinite"/>
                </circle>
                <circle cx="62" cy="35" r="7" fill="#00ff88">
                    <animate attributeName="opacity" values="1;0.4;1" dur="3s" repeatCount="indefinite"/>
                </circle>
                <circle cx="38" cy="35" r="2.5" fill="#0d0d0d"/>
                <circle cx="62" cy="35" r="2.5" fill="#0d0d0d"/>
                <ellipse cx="50" cy="14" rx="20" ry="7" fill="#1a1a1a" stroke="#ff0040" stroke-width="1"/>
                <path d="M35 14 Q50 -5 65 14" fill="#1a1a1a" stroke="#ff0040" stroke-width="1"/>
            </svg>
        </div>
        <div class="header-info">
            <h1>Dr Marshall</h1>
            <p>OSINT Intelligence Assistant • Security Researcher</p>
        </div>
        <div class="header-actions">
            <button class="header-btn" onclick="toggleAISettings()">
                <span>⚙️</span> AI Models
            </button>
            <button class="header-btn" onclick="window.location.href='marshall://osint'">
                <span>🔍</span> OSINT Tools
            </button>
            <button class="header-btn" onclick="window.location.href='marshall://home'">
                <span>🏠</span> Home
            </button>
            <div class="status-indicator" id="statusIndicator">
                <div class="status-dot" id="statusDot"></div>
                <span id="statusText">Local Mode</span>
            </div>
        </div>
    </div>
    <!-- AI Settings Modal -->
    <div class="modal-overlay" id="aiSettingsModal" style="display:none;">
        <div class="modal">
            <div class="modal-header">
                <h2>🤖 AI Model Configuration</h2>
                <button class="modal-close" onclick="toggleAISettings()">×</button>
            </div>
            <div class="modal-body">
                <div class="settings-group">
                    <label>AI Provider</label>
                    <select id="aiProvider" onchange="updateProviderFields()">
                        <option value="local">Local (Pattern Matching)</option>
                        <option value="ollama">Ollama (Local LLM)</option>
                        <option value="openai">OpenAI Compatible</option>
                        <option value="groq">Groq (Free Tier)</option>
                        <option value="together">Together AI</option>
                        <option value="openrouter">OpenRouter</option>
                        <option value="custom">Custom Endpoint</option>
                    </select>
                </div>
                <div id="providerFields">
                    <div class="settings-group" id="endpointGroup" style="display:none;">
                        <label>API Endpoint</label>
                        <input type="text" id="apiEndpoint" placeholder="http://localhost:11434/api/generate">
                    </div>
                    <div class="settings-group" id="apiKeyGroup" style="display:none;">
                        <label>API Key</label>
                        <input type="password" id="apiKey" placeholder="sk-...">
                        <small>Your key is stored locally and never transmitted elsewhere</small>
                    </div>
                    <div class="settings-group" id="modelGroup" style="display:none;">
                        <label>Model</label>
                        <select id="modelSelect">
                            <option value="">Select a model...</option>
                        </select>
                        <input type="text" id="customModel" placeholder="Or enter custom model name" style="margin-top:8px;">
                    </div>
                </div>
                <div class="settings-group">
                    <label>System Prompt</label>
                    <textarea id="systemPrompt" rows="4" placeholder="You are Dr Marshall, an OSINT and security research assistant..."></textarea>
                </div>
                <div class="settings-group">
                    <label>Temperature</label>
                    <div style="display:flex;align-items:center;gap:12px;">
                        <input type="range" id="temperature" min="0" max="100" value="70" style="flex:1;">
                        <span id="tempValue">0.7</span>
                    </div>
                </div>
                <div class="info-box info" style="margin-top:16px;">
                    💡 <strong>Free AI Options:</strong><br>
                    • <strong>Ollama</strong> - Run models locally (llama3, mistral, codellama)<br>
                    • <strong>Groq</strong> - Free tier with llama-3.1-70b, mixtral-8x7b<br>
                    • <strong>Together AI</strong> - Free credits for open models<br>
                    • <strong>OpenRouter</strong> - Access to many free models
                </div>
            </div>
            <div class="modal-footer">
                <button class="modal-btn secondary" onclick="testConnection()">Test Connection</button>
                <button class="modal-btn primary" onclick="saveAISettings()">Save Settings</button>
            </div>
        </div>
    </div>
    <div class="chat-container" id="chat">
        <div class="message message-assistant">
            <div class="message-content">
                <strong style="font-size: 16px;">Welcome, Operator.</strong><br><br>
                I'm <strong>Dr Marshall</strong>, your private intelligence assistant. I specialize in:
                <ul>
                    <li><strong>OSINT Research</strong> - Domain intel, WHOIS, DNS analysis</li>
                    <li><strong>Network Reconnaissance</strong> - Port scanning, service enumeration</li>
                    <li><strong>Vulnerability Research</strong> - CVE lookups, exploit information</li>
                    <li><strong>Social Engineering</strong> - Email tracing, username searches</li>
                    <li><strong>Privacy & OpSec</strong> - Anonymity techniques, secure practices</li>
                </ul>
                <div class="info-box info">
                    💡 <strong>Tip:</strong> Try asking about a specific domain, IP address, or security topic. I can provide detailed analysis and actionable intelligence.
                </div>
                <div class="suggestions">
                    <span class="suggestion" onclick="setQuery('Analyze domain github.com')">🌐 Analyze domain</span>
                    <span class="suggestion" onclick="setQuery('What info can I get from an IP address?')">📍 IP Intelligence</span>
                    <span class="suggestion" onclick="setQuery('How do I find subdomains?')">🔎 Subdomain enum</span>
                    <span class="suggestion" onclick="setQuery('OSINT tools for email investigation')">📧 Email OSINT</span>
                </div>
            </div>
            <div class="message-meta">Dr Marshall • Ready to assist</div>
        </div>
    </div>
    <div class="input-area">
        <form class="input-form" onsubmit="sendMessage(event)">
            <input type="text" class="chat-input" id="userInput" placeholder="Ask me about OSINT, security, or enter a target to analyze..." autofocus>
            <button type="submit" class="send-btn">Send</button>
        </form>
        <div class="quick-actions">
            <button class="quick-btn" onclick="setQuery('WHOIS lookup for ')"><span class="icon">🔍</span> WHOIS</button>
            <button class="quick-btn" onclick="setQuery('DNS records for ')"><span class="icon">📋</span> DNS</button>
            <button class="quick-btn" onclick="setQuery('Common ports for ')"><span class="icon">🔌</span> Ports</button>
            <button class="quick-btn" onclick="setQuery('Find CVEs for ')"><span class="icon">🛡️</span> CVE Search</button>
            <button class="quick-btn" onclick="setQuery('Username search for ')"><span class="icon">👤</span> Username</button>
            <button class="quick-btn" onclick="clearChat()"><span class="icon">🗑️</span> Clear</button>
        </div>
    </div>
    <script>
        // AI Configuration
        var aiConfig = {
            provider: 'local',
            endpoint: '',
            apiKey: '',
            model: '',
            systemPrompt: 'You are Dr Marshall, an expert OSINT and cybersecurity research assistant. You help users with domain intelligence, network reconnaissance, vulnerability research, and privacy/OpSec guidance. Be concise, technical, and actionable. Format responses with clear structure using HTML tags like <strong>, <ul>, <li>, <code>, and <pre>.',
            temperature: 0.7
        };
        
        var conversationContext = [];
        var providerConfigs = {
            local: { name: 'Local', needsKey: false, needsEndpoint: false, models: [] },
            ollama: { 
                name: 'Ollama', 
                needsKey: false, 
                needsEndpoint: true, 
                defaultEndpoint: 'http://localhost:11434/api/generate',
                models: ['llama3.2', 'llama3.1', 'mistral', 'codellama', 'mixtral', 'phi3', 'qwen2.5']
            },
            openai: { 
                name: 'OpenAI Compatible', 
                needsKey: true, 
                needsEndpoint: true,
                defaultEndpoint: 'https://api.openai.com/v1/chat/completions',
                models: ['gpt-4o-mini', 'gpt-4o', 'gpt-3.5-turbo']
            },
            groq: { 
                name: 'Groq (Free)', 
                needsKey: true, 
                needsEndpoint: false,
                defaultEndpoint: 'https://api.groq.com/openai/v1/chat/completions',
                models: ['llama-3.1-70b-versatile', 'llama-3.1-8b-instant', 'mixtral-8x7b-32768', 'gemma2-9b-it']
            },
            together: { 
                name: 'Together AI', 
                needsKey: true, 
                needsEndpoint: false,
                defaultEndpoint: 'https://api.together.xyz/v1/chat/completions',
                models: ['meta-llama/Meta-Llama-3.1-70B-Instruct-Turbo', 'mistralai/Mixtral-8x7B-Instruct-v0.1', 'Qwen/Qwen2.5-72B-Instruct-Turbo']
            },
            openrouter: { 
                name: 'OpenRouter', 
                needsKey: true, 
                needsEndpoint: false,
                defaultEndpoint: 'https://openrouter.ai/api/v1/chat/completions',
                models: ['google/gemini-2.0-flash-exp:free', 'meta-llama/llama-3.1-8b-instruct:free', 'mistralai/mistral-7b-instruct:free', 'qwen/qwen-2-7b-instruct:free']
            },
            custom: { 
                name: 'Custom', 
                needsKey: true, 
                needsEndpoint: true,
                defaultEndpoint: '',
                models: []
            }
        };
        
        // Load saved settings
        function loadAISettings() {
            try {
                var saved = localStorage.getItem('drMarshallAI');
                if (saved) {
                    var parsed = JSON.parse(saved);
                    Object.assign(aiConfig, parsed);
                    updateUIFromConfig();
                }
            } catch(e) { console.log('Failed to load settings:', e); }
            updateStatusIndicator();
        }
        
        function updateUIFromConfig() {
            document.getElementById('aiProvider').value = aiConfig.provider;
            document.getElementById('apiEndpoint').value = aiConfig.endpoint;
            document.getElementById('apiKey').value = aiConfig.apiKey;
            document.getElementById('customModel').value = aiConfig.model;
            document.getElementById('systemPrompt').value = aiConfig.systemPrompt;
            document.getElementById('temperature').value = aiConfig.temperature * 100;
            document.getElementById('tempValue').textContent = aiConfig.temperature.toFixed(1);
            updateProviderFields();
        }
        
        function updateProviderFields() {
            var provider = document.getElementById('aiProvider').value;
            var config = providerConfigs[provider];
            
            document.getElementById('endpointGroup').style.display = config.needsEndpoint ? 'block' : 'none';
            document.getElementById('apiKeyGroup').style.display = config.needsKey ? 'block' : 'none';
            document.getElementById('modelGroup').style.display = provider !== 'local' ? 'block' : 'none';
            
            // Set default endpoint
            if (config.defaultEndpoint && !document.getElementById('apiEndpoint').value) {
                document.getElementById('apiEndpoint').value = config.defaultEndpoint;
            }
            
            // Populate models
            var modelSelect = document.getElementById('modelSelect');
            modelSelect.innerHTML = '<option value="">Select a model...</option>';
            config.models.forEach(function(m) {
                var opt = document.createElement('option');
                opt.value = m;
                opt.textContent = m;
                if (m === aiConfig.model) opt.selected = true;
                modelSelect.appendChild(opt);
            });
        }
        
        function toggleAISettings() {
            var modal = document.getElementById('aiSettingsModal');
            modal.style.display = modal.style.display === 'none' ? 'flex' : 'none';
            if (modal.style.display === 'flex') {
                updateUIFromConfig();
            }
        }
        
        function saveAISettings() {
            aiConfig.provider = document.getElementById('aiProvider').value;
            aiConfig.endpoint = document.getElementById('apiEndpoint').value || providerConfigs[aiConfig.provider].defaultEndpoint || '';
            aiConfig.apiKey = document.getElementById('apiKey').value;
            aiConfig.model = document.getElementById('customModel').value || document.getElementById('modelSelect').value;
            aiConfig.systemPrompt = document.getElementById('systemPrompt').value || providerConfigs.local.systemPrompt;
            aiConfig.temperature = parseInt(document.getElementById('temperature').value) / 100;
            
            localStorage.setItem('drMarshallAI', JSON.stringify(aiConfig));
            updateStatusIndicator();
            toggleAISettings();
        }
        
        function updateStatusIndicator() {
            var indicator = document.getElementById('statusIndicator');
            var text = document.getElementById('statusText');
            indicator.className = 'status-indicator';
            
            if (aiConfig.provider === 'local') {
                indicator.classList.add('status-local');
                text.textContent = 'Local Mode';
            } else {
                indicator.classList.add('status-connected');
                text.textContent = providerConfigs[aiConfig.provider].name + ' (' + (aiConfig.model || 'default') + ')';
            }
        }
        
        async function testConnection() {
            if (aiConfig.provider === 'local') {
                alert('Local mode uses built-in pattern matching - no connection needed!');
                return;
            }
            
            var btn = event.target;
            btn.textContent = 'Testing...';
            btn.disabled = true;
            
            try {
                var response = await callAI('Hello, are you working?');
                alert('✓ Connection successful!\\n\\nResponse: ' + response.substring(0, 200) + '...');
                updateStatusIndicator();
            } catch(e) {
                alert('✗ Connection failed:\\n' + e.message);
                var indicator = document.getElementById('statusIndicator');
                indicator.className = 'status-indicator status-error';
                document.getElementById('statusText').textContent = 'Error';
            }
            
            btn.textContent = 'Test Connection';
            btn.disabled = false;
        }
        
        // Temperature slider
        document.getElementById('temperature').addEventListener('input', function() {
            document.getElementById('tempValue').textContent = (this.value / 100).toFixed(1);
        });
        
        function setQuery(text) {
            var input = document.getElementById('userInput');
            input.value = text;
            input.focus();
            input.setSelectionRange(text.length, text.length);
        }
        
        function clearChat() {
            var chat = document.getElementById('chat');
            chat.innerHTML = '<div class="message message-assistant"><div class="message-content"><strong>Chat cleared.</strong> How can I help you?</div><div class="message-meta">Dr Marshall • Ready</div></div>';
            conversationContext = [];
        }
        
        async function sendMessage(e) {
            e.preventDefault();
            var input = document.getElementById('userInput');
            var query = input.value.trim();
            if (!query) return;
            
            var chat = document.getElementById('chat');
            var time = new Date().toLocaleTimeString([], {hour: '2-digit', minute:'2-digit'});
            
            // Add user message
            chat.innerHTML += '<div class="message message-user"><div class="message-content">' + escapeHtml(query) + '</div><div class="message-meta">You • ' + time + '</div></div>';
            input.value = '';
            
            // Add typing indicator
            var typingId = 'typing-' + Date.now();
            chat.innerHTML += '<div class="message message-assistant" id="' + typingId + '"><div class="typing-indicator"><div class="typing-dot"></div><div class="typing-dot"></div><div class="typing-dot"></div></div></div>';
            chat.scrollTop = chat.scrollHeight;
            
            // Store context
            conversationContext.push({role: 'user', content: query});
            
            // Get response (AI or local)
            var response;
            if (aiConfig.provider === 'local') {
                await new Promise(r => setTimeout(r, 600 + Math.random() * 800));
                response = generateLocalResponse(query);
            } else {
                try {
                    response = await callAI(query);
                } catch(e) {
                    response = '<div class="info-box warning">⚠️ AI connection failed: ' + escapeHtml(e.message) + '</div>' + generateLocalResponse(query);
                }
            }
            
            var typing = document.getElementById(typingId);
            if (typing) typing.remove();
            
            conversationContext.push({role: 'assistant', content: response});
            
            chat.innerHTML += '<div class="message message-assistant"><div class="message-content">' + response + '</div><div class="message-meta">Dr Marshall • ' + time + '</div></div>';
            chat.scrollTop = chat.scrollHeight;
        }
        
        async function callAI(query) {
            var endpoint = aiConfig.endpoint || providerConfigs[aiConfig.provider].defaultEndpoint;
            var headers = { 'Content-Type': 'application/json' };
            
            if (aiConfig.apiKey) {
                headers['Authorization'] = 'Bearer ' + aiConfig.apiKey;
            }
            
            // Build messages array with context
            var messages = [{ role: 'system', content: aiConfig.systemPrompt }];
            var contextToSend = conversationContext.slice(-10); // Last 10 messages
            contextToSend.forEach(function(msg) {
                messages.push({ role: msg.role, content: msg.content });
            });
            messages.push({ role: 'user', content: query });
            
            var body;
            if (aiConfig.provider === 'ollama') {
                // Ollama format
                body = JSON.stringify({
                    model: aiConfig.model || 'llama3.2',
                    prompt: aiConfig.systemPrompt + '\\n\\nUser: ' + query,
                    stream: false,
                    options: { temperature: aiConfig.temperature }
                });
            } else {
                // OpenAI-compatible format (Groq, Together, OpenRouter, etc.)
                body = JSON.stringify({
                    model: aiConfig.model,
                    messages: messages,
                    temperature: aiConfig.temperature,
                    max_tokens: 2000
                });
            }
            
            var response = await fetch(endpoint, {
                method: 'POST',
                headers: headers,
                body: body
            });
            
            if (!response.ok) {
                var errorText = await response.text();
                throw new Error('API error ' + response.status + ': ' + errorText.substring(0, 200));
            }
            
            var data = await response.json();
            
            // Extract response based on provider format
            if (aiConfig.provider === 'ollama') {
                return formatAIResponse(data.response || '');
            } else {
                return formatAIResponse(data.choices?.[0]?.message?.content || '');
            }
        }
        
        function formatAIResponse(text) {
            // Convert markdown-style formatting to HTML
            text = text.replace(/\\*\\*(.+?)\\*\\*/g, '<strong>$1</strong>');
            text = text.replace(/\\*(.+?)\\*/g, '<em>$1</em>');
            text = text.replace(/`([^`]+)`/g, '<code>$1</code>');
            text = text.replace(/```([\\s\\S]*?)```/g, '<pre><code>$1</code></pre>');
            text = text.replace(/^- (.+)$/gm, '<li>$1</li>');
            text = text.replace(/(<li>.*<\\/li>)/s, '<ul>$1</ul>');
            text = text.replace(/\\n/g, '<br>');
            return text;
        }
        
        function escapeHtml(text) {
            var div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        }
        
        function extractTarget(query) {
            // Extract domain, IP, or username from query
            var domainMatch = query.match(/(?:analyze|lookup|check|scan|whois|dns|info|about|for)\s+(?:domain\s+)?([a-zA-Z0-9][-a-zA-Z0-9]*(?:\.[a-zA-Z0-9][-a-zA-Z0-9]*)+)/i);
            var ipMatch = query.match(/(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})/);
            var usernameMatch = query.match(/(?:username|user|find)\s+(?:search\s+)?(?:for\s+)?["']?(\w+)["']?/i);
            
            if (domainMatch) return {type: 'domain', value: domainMatch[1]};
            if (ipMatch) return {type: 'ip', value: ipMatch[1]};
            if (usernameMatch) return {type: 'username', value: usernameMatch[1]};
            return null;
        }
        
        function generateLocalResponse(query) {
            var q = query.toLowerCase();
            var target = extractTarget(query);
            
            // Domain analysis
            if (target && target.type === 'domain') {
                return generateDomainResponse(target.value);
            }
            
            // IP analysis
            if (target && target.type === 'ip') {
                return generateIPResponse(target.value);
            }
            
            // Username search
            if (target && target.type === 'username') {
                return generateUsernameResponse(target.value);
            }
            
            // WHOIS queries
            if (q.includes('whois')) {
                return '<strong>🔍 WHOIS Lookup</strong><br><br>WHOIS provides domain registration data:<ul><li><strong>Registrar</strong> - Where the domain was registered</li><li><strong>Creation/Expiry dates</strong> - Domain age and renewal</li><li><strong>Nameservers</strong> - DNS infrastructure</li><li><strong>Registrant info</strong> - Owner details (if not private)</li></ul><div class="info-box info">Enter a domain after "WHOIS lookup for" or use our <a href="javascript:void(0)" onclick="window.location.href=\'marshall://osint\'">OSINT Tools</a> for detailed lookups.</div><strong>External resources:</strong><br>• <a href="https://who.is" target="_blank">who.is</a><br>• <a href="https://whois.domaintools.com" target="_blank">DomainTools</a>';
            }
            
            // DNS queries
            if (q.includes('dns') || q.includes('nameserver') || q.includes('record')) {
                return '<strong>📋 DNS Records Guide</strong><br><br><table style="width:100%;border-collapse:collapse;margin:10px 0;"><tr style="border-bottom:1px solid var(--border);"><td style="padding:8px;"><code>A</code></td><td>IPv4 address mapping</td></tr><tr style="border-bottom:1px solid var(--border);"><td style="padding:8px;"><code>AAAA</code></td><td>IPv6 address mapping</td></tr><tr style="border-bottom:1px solid var(--border);"><td style="padding:8px;"><code>MX</code></td><td>Mail server records</td></tr><tr style="border-bottom:1px solid var(--border);"><td style="padding:8px;"><code>TXT</code></td><td>SPF, DKIM, verification</td></tr><tr style="border-bottom:1px solid var(--border);"><td style="padding:8px;"><code>CNAME</code></td><td>Canonical name aliases</td></tr><tr><td style="padding:8px;"><code>NS</code></td><td>Nameserver delegation</td></tr></table><pre><code>dig example.com ANY\nnslookup -type=any example.com</code></pre>';
            }
            
            // Port scanning
            if (q.includes('port') || q.includes('scan') || q.includes('nmap')) {
                return '<strong>🔌 Port Scanning Guide</strong><br><br><strong>Common ports to check:</strong><table style="width:100%;margin:10px 0;"><tr><td style="padding:4px;"><span class="tag">21</span> FTP</td><td style="padding:4px;"><span class="tag">22</span> SSH</td><td style="padding:4px;"><span class="tag">23</span> Telnet</td></tr><tr><td style="padding:4px;"><span class="tag">25</span> SMTP</td><td style="padding:4px;"><span class="tag">53</span> DNS</td><td style="padding:4px;"><span class="tag">80</span> HTTP</td></tr><tr><td style="padding:4px;"><span class="tag">443</span> HTTPS</td><td style="padding:4px;"><span class="tag">3306</span> MySQL</td><td style="padding:4px;"><span class="tag">3389</span> RDP</td></tr></table><pre><code># Quick scan\nnmap -F target.com\n\n# Service version detection\nnmap -sV -sC target.com\n\n# Full port scan\nnmap -p- target.com</code></pre><div class="info-box warning">⚠️ Only scan systems you have permission to test.</div>';
            }
            
            // CVE/Vulnerability
            if (q.includes('cve') || q.includes('vulnerab') || q.includes('exploit')) {
                return '<strong>🛡️ Vulnerability Research</strong><br><br>Resources for CVE and exploit information:<ul><li><a href="https://nvd.nist.gov/" target="_blank">NVD (NIST)</a> - Official CVE database</li><li><a href="https://cve.mitre.org/" target="_blank">MITRE CVE</a> - CVE identifiers</li><li><a href="https://www.exploit-db.com/" target="_blank">Exploit-DB</a> - Exploit archive</li><li><a href="https://vulners.com/" target="_blank">Vulners</a> - Vulnerability search</li></ul><pre><code># Search format\nCVE-YYYY-NNNNN\n\n# Example\nCVE-2021-44228 (Log4Shell)</code></pre><div class="info-box info">💡 Always verify CVE applicability to your specific software version.</div>';
            }
            
            // Subdomain enumeration
            if (q.includes('subdomain')) {
                return '<strong>🔎 Subdomain Enumeration</strong><br><br>Methods to discover subdomains:<ul><li><strong>Passive</strong> - Certificate logs, DNS records, search engines</li><li><strong>Active</strong> - Brute force, zone transfers</li></ul><pre><code># Using subfinder\nsubfinder -d target.com\n\n# Using amass\namass enum -d target.com\n\n# Certificate search\ncurl "https://crt.sh/?q=%.target.com&output=json"</code></pre><strong>Online tools:</strong><br>• <a href="https://crt.sh" target="_blank">crt.sh</a> - Certificate transparency<br>• <a href="https://dnsdumpster.com" target="_blank">DNSDumpster</a><br>• <a href="https://securitytrails.com" target="_blank">SecurityTrails</a>';
            }
            
            // Email OSINT
            if (q.includes('email')) {
                return '<strong>📧 Email OSINT</strong><br><br>Investigate email addresses with:<ul><li><a href="https://haveibeenpwned.com" target="_blank">HaveIBeenPwned</a> - Breach database</li><li><a href="https://hunter.io" target="_blank">Hunter.io</a> - Email finder/verifier</li><li><a href="https://emailrep.io" target="_blank">EmailRep</a> - Reputation check</li></ul><strong>Header analysis:</strong><pre><code>Received: from [origin server]\nX-Originating-IP: [sender IP]\nMessage-ID: [unique identifier]</code></pre><div class="info-box info">💡 Check email headers for origin servers and routing information.</div>';
            }
            
            // Username search
            if (q.includes('username') || q.includes('user search') || q.includes('social media')) {
                return '<strong>👤 Username Search</strong><br><br>Find accounts across platforms:<ul><li><a href="https://namechk.com" target="_blank">Namechk</a> - Username availability</li><li><a href="https://whatsmyname.app" target="_blank">WhatsMyName</a> - Profile search</li><li><a href="https://sherlock-project.github.io" target="_blank">Sherlock</a> - CLI tool</li></ul><pre><code># Using Sherlock\npython3 sherlock username\n\n# Check specific sites\npython3 sherlock --site twitter username</code></pre>';
            }
            
            // IP address info
            if (q.includes('ip address') || q.includes('ip intel') || q.includes('geolocation')) {
                return '<strong>📍 IP Intelligence</strong><br><br>Information available from an IP address:<ul><li><strong>Geolocation</strong> - Country, city, coordinates</li><li><strong>ASN/ISP</strong> - Network ownership</li><li><strong>Reverse DNS</strong> - Associated hostnames</li><li><strong>Reputation</strong> - Blacklist status, threat intel</li></ul><strong>Tools:</strong><br>• <a href="https://ipinfo.io" target="_blank">ipinfo.io</a><br>• <a href="https://shodan.io" target="_blank">Shodan</a><br>• <a href="https://abuseipdb.com" target="_blank">AbuseIPDB</a>';
            }
            
            // OpSec/Privacy
            if (q.includes('opsec') || q.includes('privacy') || q.includes('anonym')) {
                return '<strong>🔒 OpSec & Privacy</strong><br><br><strong>Key principles:</strong><ul><li>Compartmentalize identities and activities</li><li>Use VPN/Tor for sensitive research</li><li>Avoid cross-contamination of personas</li><li>Minimize metadata in shared files</li></ul><div class="info-box warning">⚠️ <strong>Remember:</strong> Your browser fingerprint, timing patterns, and writing style can identify you.</div><strong>Tools:</strong><br>• Tor Browser, Tails OS<br>• ProtonMail, Signal<br>• VPN with no-logs policy';
            }
            
            // Shodan
            if (q.includes('shodan')) {
                return '<strong>🌐 Shodan Search Engine</strong><br><br>Shodan indexes internet-connected devices:<pre><code># Search queries\nport:22 country:US\napache city:"New York"\nssl.cert.subject.cn:target.com\nvuln:CVE-2021-44228</code></pre><a href="https://shodan.io" target="_blank">shodan.io</a> - Requires account for full access<br><br><div class="info-box info">💡 Shodan CLI: <code>shodan search apache</code></div>';
            }
            
            // Help/capabilities
            if (q.includes('help') || q.includes('what can you') || q.includes('capabilities')) {
                return '<strong>🤖 Dr Marshall Capabilities</strong><br><br>I can assist with:<ul><li>🌐 <strong>Domain Analysis</strong> - WHOIS, DNS, subdomains, certificates</li><li>📍 <strong>IP Intelligence</strong> - Geolocation, ASN, reputation</li><li>🔌 <strong>Network Recon</strong> - Port scanning, service detection</li><li>🛡️ <strong>Vulnerability Research</strong> - CVE lookups, exploit info</li><li>👤 <strong>People OSINT</strong> - Username search, email investigation</li><li>🔒 <strong>OpSec Guidance</strong> - Privacy and anonymity advice</li></ul><div class="suggestions"><span class="suggestion" onclick="setQuery(\'Analyze domain example.com\')">Try: Analyze a domain</span><span class="suggestion" onclick="setQuery(\'How do I stay anonymous?\')">Try: OpSec tips</span></div>';
            }
            
            // Default intelligent response
            return '<strong>Analysis Request</strong><br><br>I can help you investigate <em>"' + escapeHtml(query) + '"</em>.<br><br>To provide specific intelligence, try:<ul><li>"Analyze domain <strong>target.com</strong>"</li><li>"WHOIS lookup for <strong>target.com</strong>"</li><li>"What ports are open on <strong>192.168.1.1</strong>"</li><li>"Find username <strong>johndoe</strong>"</li></ul><div class="suggestions"><span class="suggestion" onclick="window.location.href=\'marshall://osint\'">Open OSINT Tools</span><span class="suggestion" onclick="setQuery(\'help\')">Show capabilities</span></div>';
        }
        
        function generateDomainResponse(domain) {
            return '<strong>🌐 Domain Analysis: ' + escapeHtml(domain) + '</strong><br><br><div class="info-box success">✓ Target identified. Gathering intelligence...</div><strong>Recommended reconnaissance:</strong><ol><li><strong>WHOIS</strong> - Registration data, registrar, dates</li><li><strong>DNS Records</strong> - A, MX, TXT, NS records</li><li><strong>Subdomains</strong> - Discover additional attack surface</li><li><strong>SSL Certificate</strong> - Check validity, issuer, SANs</li><li><strong>Web Technologies</strong> - Server, frameworks, CMS</li></ol><pre><code>whois ' + escapeHtml(domain) + '\ndig ' + escapeHtml(domain) + ' ANY\ncurl -I https://' + escapeHtml(domain) + '</code></pre><div class="suggestions"><span class="suggestion" onclick="window.location.href=\'marshall://osint/' + encodeURIComponent(domain) + '\'">🔍 Full OSINT Scan</span><span class="suggestion" onclick="setQuery(\'Find subdomains of ' + escapeHtml(domain) + '\')">📋 Subdomains</span></div>';
        }
        
        function generateIPResponse(ip) {
            return '<strong>📍 IP Analysis: ' + escapeHtml(ip) + '</strong><br><br><div class="info-box success">✓ IP address identified.</div><strong>Intelligence gathering:</strong><ul><li><strong>Geolocation</strong> - Physical location estimate</li><li><strong>ASN/ISP</strong> - Network ownership</li><li><strong>Reverse DNS</strong> - Associated hostnames</li><li><strong>Port scan</strong> - Open services</li><li><strong>Reputation</strong> - Blacklist/threat status</li></ul><pre><code>nslookup ' + escapeHtml(ip) + '\nwhois ' + escapeHtml(ip) + '\nnmap -sV ' + escapeHtml(ip) + '</code></pre><strong>Check online:</strong><br>• <a href="https://ipinfo.io/' + escapeHtml(ip) + '" target="_blank">ipinfo.io/' + escapeHtml(ip) + '</a><br>• <a href="https://shodan.io/host/' + escapeHtml(ip) + '" target="_blank">Shodan</a>';
        }
        
        function generateUsernameResponse(username) {
            return '<strong>👤 Username Search: ' + escapeHtml(username) + '</strong><br><br><div class="info-box success">✓ Searching for accounts...</div><strong>Platforms to check:</strong><ul><li>Social: Twitter, Instagram, Facebook, LinkedIn</li><li>Dev: GitHub, GitLab, Stack Overflow</li><li>Forums: Reddit, HackerNews, Discord</li><li>Gaming: Steam, Twitch, Xbox</li></ul><pre><code># Using Sherlock\npython3 sherlock ' + escapeHtml(username) + '\n\n# Manual checks\nhttps://twitter.com/' + escapeHtml(username) + '\nhttps://github.com/' + escapeHtml(username) + '</code></pre><strong>Tools:</strong><br>• <a href="https://whatsmyname.app" target="_blank">WhatsMyName</a><br>• <a href="https://namechk.com" target="_blank">Namechk</a>';
        }
        
        // Initialize on page load
        document.addEventListener('DOMContentLoaded', function() {
            loadAISettings();
        });
    </script>
</body>
</html>"##.to_string()
}

/// Generate OSINT Tools page
pub fn generate_osint_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Marshall OSINT Tools</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }
        a { color: var(--red); text-decoration: none; }
        a:hover { text-decoration: underline; }
        .back-link {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color: var(--fg-dim);
            font-size: 14px;
            margin: 1rem 2rem;
        }
        .header {
            background: linear-gradient(135deg, var(--bg2) 0%, var(--bg) 100%);
            border-bottom: 2px solid var(--red);
            padding: 2rem;
            text-align: center;
        }
        .header h1 { font-size: 28px; color: var(--red); margin-bottom: 0.5rem; }
        .header p { color: var(--fg-dim); }
        .search-section {
            max-width: 600px;
            margin: 2rem auto;
            padding: 0 2rem;
        }
        .search-form {
            display: flex;
            gap: 12px;
        }
        .search-form input {
            flex: 1;
            padding: 14px 18px;
            background: var(--bg2);
            border: 2px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 15px;
            outline: none;
        }
        .search-form input:focus { border-color: var(--red); }
        .search-form button {
            padding: 14px 28px;
            background: var(--red);
            border: none;
            border-radius: 8px;
            color: white;
            font-weight: 600;
            cursor: pointer;
        }
        .tools-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
            gap: 1.5rem;
            padding: 2rem;
            max-width: 1200px;
            margin: 0 auto;
        }
        .tool-card {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.5rem;
            transition: all 0.2s ease;
        }
        .tool-card:hover {
            border-color: var(--red);
            transform: translateY(-2px);
        }
        .tool-icon { font-size: 36px; margin-bottom: 1rem; }
        .tool-title { font-size: 18px; font-weight: 600; color: var(--red); margin-bottom: 0.5rem; }
        .tool-desc { color: var(--fg-dim); font-size: 13px; margin-bottom: 1rem; line-height: 1.5; }
        .tool-actions { display: flex; gap: 8px; flex-wrap: wrap; }
        .tool-btn {
            padding: 8px 16px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 6px;
            color: var(--fg);
            font-size: 12px;
            cursor: pointer;
            text-decoration: none;
        }
        .tool-btn:hover {
            border-color: var(--red);
            color: var(--red);
            text-decoration: none;
        }
    </style>
</head>
<body>
    <a href="javascript:void(0)" onclick="window.location.href='marshall://home'" class="back-link">&#8592; Back to Home</a>
    <div class="header">
        <h1>OSINT Tools</h1>
        <p>Intelligence Gathering and Security Analysis</p>
    </div>
    <div class="search-section">
        <form class="search-form" onsubmit="event.preventDefault(); if(this.target.value) location.href='marshall://osint/'+encodeURIComponent(this.target.value);">
            <input type="text" name="target" placeholder="Enter domain, IP, or email..." autofocus>
            <button type="submit">Analyze</button>
        </form>
    </div>
    <div class="tools-grid">
        <div class="tool-card">
            <div class="tool-icon">&#127760;</div>
            <div class="tool-title">WHOIS Lookup</div>
            <div class="tool-desc">Get domain registration information, registrar details, and nameservers.</div>
            <div class="tool-actions">
                <a href="https://who.is/" target="_blank" class="tool-btn">who.is</a>
                <a href="https://whois.domaintools.com/" target="_blank" class="tool-btn">DomainTools</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#128203;</div>
            <div class="tool-title">DNS Records</div>
            <div class="tool-desc">Query A, MX, TXT, NS and other DNS records for a domain.</div>
            <div class="tool-actions">
                <a href="https://dnsdumpster.com/" target="_blank" class="tool-btn">DNSDumpster</a>
                <a href="https://mxtoolbox.com/" target="_blank" class="tool-btn">MXToolbox</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#128268;</div>
            <div class="tool-title">Port Scanner</div>
            <div class="tool-desc">Discover open ports and running services on target hosts.</div>
            <div class="tool-actions">
                <a href="https://www.shodan.io/" target="_blank" class="tool-btn">Shodan</a>
                <a href="https://censys.io/" target="_blank" class="tool-btn">Censys</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#128274;</div>
            <div class="tool-title">SSL/TLS Analysis</div>
            <div class="tool-desc">Check certificate validity and security configuration.</div>
            <div class="tool-actions">
                <a href="https://www.ssllabs.com/ssltest/" target="_blank" class="tool-btn">SSL Labs</a>
                <a href="https://crt.sh/" target="_blank" class="tool-btn">crt.sh</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#128231;</div>
            <div class="tool-title">Email OSINT</div>
            <div class="tool-desc">Research email addresses and find data breaches.</div>
            <div class="tool-actions">
                <a href="https://haveibeenpwned.com/" target="_blank" class="tool-btn">HIBP</a>
                <a href="https://epieos.com/" target="_blank" class="tool-btn">Epieos</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#128100;</div>
            <div class="tool-title">Username Search</div>
            <div class="tool-desc">Find social media profiles across platforms.</div>
            <div class="tool-actions">
                <a href="https://namechk.com/" target="_blank" class="tool-btn">Namechk</a>
                <a href="https://whatsmyname.app/" target="_blank" class="tool-btn">WhatsMyName</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#9888;</div>
            <div class="tool-title">Vulnerability Database</div>
            <div class="tool-desc">Search for known CVEs and security advisories.</div>
            <div class="tool-actions">
                <a href="https://nvd.nist.gov/" target="_blank" class="tool-btn">NVD</a>
                <a href="https://www.exploit-db.com/" target="_blank" class="tool-btn">Exploit-DB</a>
            </div>
        </div>
        <div class="tool-card">
            <div class="tool-icon">&#128260;</div>
            <div class="tool-title">Reverse Lookups</div>
            <div class="tool-desc">Reverse IP, DNS, and image searches.</div>
            <div class="tool-actions">
                <a href="https://viewdns.info/" target="_blank" class="tool-btn">ViewDNS</a>
                <a href="https://securitytrails.com/" target="_blank" class="tool-btn">SecurityTrails</a>
            </div>
        </div>
    </div>
</body>
</html>"##.to_string()
}

/// Generate Settings page
pub fn generate_settings_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Marshall Settings</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }
        a { color: var(--red); text-decoration: none; }
        .back-link {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color: var(--fg-dim);
            font-size: 14px;
            margin: 1rem 2rem;
        }
        .header {
            background: var(--bg2);
            border-bottom: 1px solid var(--border);
            padding: 2rem;
        }
        .header h1 { font-size: 24px; color: var(--red); }
        .settings-container {
            max-width: 700px;
            margin: 2rem auto;
            padding: 0 2rem;
        }
        .settings-section {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 12px;
            margin-bottom: 1.5rem;
            overflow: hidden;
        }
        .section-title {
            padding: 1rem 1.5rem;
            background: var(--bg3);
            border-bottom: 1px solid var(--border);
            font-weight: 600;
            color: var(--red);
        }
        .setting-item {
            padding: 1rem 1.5rem;
            display: flex;
            align-items: center;
            justify-content: space-between;
            border-bottom: 1px solid var(--border);
        }
        .setting-item:last-child { border-bottom: none; }
        .setting-info h4 { margin-bottom: 4px; }
        .setting-info p { font-size: 12px; color: var(--fg-dim); }
        .toggle {
            width: 48px;
            height: 26px;
            background: var(--bg3);
            border-radius: 13px;
            position: relative;
            cursor: pointer;
            transition: background 0.2s ease;
        }
        .toggle.active { background: var(--green); }
        .toggle::after {
            content: '';
            position: absolute;
            width: 22px;
            height: 22px;
            background: white;
            border-radius: 50%;
            top: 2px;
            left: 2px;
            transition: transform 0.2s ease;
        }
        .toggle.active::after { transform: translateX(22px); }
        .config-btn {
            padding: 8px 16px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 12px;
            cursor: pointer;
        }
        .config-btn:hover {
            border-color: var(--red);
            color: var(--red);
        }
        .provider-list {
            display: grid;
            gap: 8px;
        }
        .provider-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 10px 12px;
            background: var(--bg3);
            border-radius: 8px;
            font-size: 13px;
        }
        .provider-name {
            font-weight: 600;
        }
        .provider-desc {
            color: var(--fg-dim);
            font-size: 11px;
        }
    </style>
</head>
<body>
    <a href="javascript:void(0)" onclick="window.location.href='marshall://home'" class="back-link">&#8592; Back to Home</a>
    <div class="header">
        <h1>Settings</h1>
    </div>
    <div class="settings-container">
        <div class="settings-section">
            <div class="section-title">Privacy</div>
            <div class="setting-item">
                <div class="setting-info">
                    <h4>Block Trackers</h4>
                    <p>Prevent third-party tracking scripts</p>
                </div>
                <div class="toggle active" onclick="this.classList.toggle('active')"></div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <h4>Block Ads</h4>
                    <p>Remove advertisements from pages</p>
                </div>
                <div class="toggle active" onclick="this.classList.toggle('active')"></div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <h4>HTTPS Only</h4>
                    <p>Always use secure connections</p>
                </div>
                <div class="toggle active" onclick="this.classList.toggle('active')"></div>
            </div>
        </div>
        <div class="settings-section">
            <div class="section-title">Dr Marshall AI</div>
            <div class="setting-item">
                <div class="setting-info">
                    <h4>Enable AI Assistant</h4>
                    <p>Access Dr Marshall for OSINT help</p>
                </div>
                <div class="toggle active" onclick="this.classList.toggle('active')"></div>
            </div>
            <div class="setting-item">
                <div class="setting-info">
                    <h4>AI Model Provider</h4>
                    <p id="currentProvider">Local (Pattern Matching)</p>
                </div>
                <button class="config-btn" onclick="window.location.href='marshall://assistant'">Configure</button>
            </div>
            <div class="setting-item" style="flex-direction:column;align-items:stretch;">
                <div class="setting-info" style="margin-bottom:12px;">
                    <h4>Available Free AI Providers</h4>
                    <p>Connect to external AI models for enhanced responses</p>
                </div>
                <div class="provider-list">
                    <div class="provider-item">
                        <span class="provider-name">🦙 Ollama</span>
                        <span class="provider-desc">Run LLMs locally - llama3, mistral, codellama</span>
                    </div>
                    <div class="provider-item">
                        <span class="provider-name">⚡ Groq</span>
                        <span class="provider-desc">Free tier - llama-3.1-70b, mixtral-8x7b</span>
                    </div>
                    <div class="provider-item">
                        <span class="provider-name">🤝 Together AI</span>
                        <span class="provider-desc">Free credits for open-source models</span>
                    </div>
                    <div class="provider-item">
                        <span class="provider-name">🌐 OpenRouter</span>
                        <span class="provider-desc">Access many free models in one API</span>
                    </div>
                </div>
            </div>
        </div>
        <div class="settings-section">
            <div class="section-title">Appearance</div>
            <div class="setting-item">
                <div class="setting-info">
                    <h4>Dark Mode</h4>
                    <p>Use dark color scheme</p>
                </div>
                <div class="toggle active" onclick="this.classList.toggle('active')"></div>
            </div>
        </div>
    </div>
</body>
</html>"##.to_string()
}

/// Generate Privacy info page
pub fn generate_privacy_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Marshall Privacy</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }
        a { color: var(--red); text-decoration: none; }
        .back-link {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color: var(--fg-dim);
            font-size: 14px;
            margin: 1rem 2rem;
        }
        .header {
            background: linear-gradient(135deg, var(--bg2) 0%, var(--bg) 100%);
            border-bottom: 2px solid var(--green);
            padding: 3rem 2rem;
            text-align: center;
        }
        .header h1 { font-size: 32px; color: var(--green); margin-bottom: 0.5rem; }
        .content {
            max-width: 700px;
            margin: 2rem auto;
            padding: 0 2rem;
        }
        .status-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 1rem;
            margin: 2rem 0;
        }
        .status-item {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 8px;
            padding: 1rem;
            text-align: center;
        }
        .status-icon { font-size: 24px; margin-bottom: 0.5rem; }
        .status-label { font-size: 12px; color: var(--fg-dim); }
        .status-value { color: var(--green); font-weight: 600; }
        .privacy-card {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-left: 3px solid var(--green);
            border-radius: 8px;
            padding: 1.5rem;
            margin-bottom: 1.5rem;
        }
        .privacy-card h3 { color: var(--green); margin-bottom: 1rem; }
        .privacy-card p { color: var(--fg-dim); line-height: 1.7; }
    </style>
</head>
<body>
    <a href="javascript:void(0)" onclick="window.location.href='marshall://home'" class="back-link">&#8592; Back to Home</a>
    <div class="header">
        <h1>Privacy Protection</h1>
        <p>Your data stays yours</p>
    </div>
    <div class="content">
        <div class="status-grid">
            <div class="status-item">
                <div class="status-icon">&#128683;</div>
                <div class="status-value">Active</div>
                <div class="status-label">Tracker Blocking</div>
            </div>
            <div class="status-item">
                <div class="status-icon">&#128274;</div>
                <div class="status-value">Enabled</div>
                <div class="status-label">HTTPS Only</div>
            </div>
            <div class="status-item">
                <div class="status-icon">&#128465;</div>
                <div class="status-value">On Exit</div>
                <div class="status-label">Auto Clear</div>
            </div>
        </div>
        <div class="privacy-card">
            <h3>No Tracking</h3>
            <p>Marshall blocks all known tracking scripts and fingerprinting attempts. We do not collect any data about your browsing habits.</p>
        </div>
        <div class="privacy-card">
            <h3>No History</h3>
            <p>By default, Marshall does not store browsing history, cookies, or cached data beyond your current session.</p>
        </div>
        <div class="privacy-card">
            <h3>Secure Connections</h3>
            <p>Marshall enforces HTTPS connections and warns you about insecure sites.</p>
        </div>
        <div class="privacy-card">
            <h3>Private AI</h3>
            <p>Dr Marshall AI runs locally. No queries are logged or used for training.</p>
        </div>
    </div>
</body>
</html>"##.to_string()
}

/// Generate OSINT results page for a specific domain
pub fn generate_osint_results(domain: &str) -> String {
    format!(r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OSINT Report - {domain}</title>
    <style>
        * {{ box-sizing: border-box; margin: 0; padding: 0; }}
        :root {{
            --red: #ff0040;
            --green: #00ff88;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }}
        html, body {{
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }}
        a {{ color: var(--red); text-decoration: none; }}
        .back-link {{
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color: var(--fg-dim);
            font-size: 14px;
            margin: 1rem 2rem;
        }}
        .header {{
            background: var(--bg2);
            border-bottom: 2px solid var(--red);
            padding: 2rem;
        }}
        .header h1 {{ color: var(--red); margin-bottom: 0.5rem; }}
        .header .target {{ font-family: monospace; color: var(--green); font-size: 18px; }}
        .container {{ max-width: 900px; margin: 2rem auto; padding: 0 2rem; }}
        .report-section {{
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 8px;
            margin-bottom: 1.5rem;
            overflow: hidden;
        }}
        .section-header {{
            background: var(--bg3);
            padding: 1rem 1.5rem;
            border-bottom: 1px solid var(--border);
            font-weight: 600;
            color: var(--red);
        }}
        .section-content {{ padding: 1.5rem; }}
        .external-links {{ display: flex; flex-wrap: wrap; gap: 8px; margin-top: 1rem; }}
        .ext-link {{
            padding: 8px 16px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 6px;
            color: var(--fg);
            font-size: 12px;
            text-decoration: none;
        }}
        .ext-link:hover {{ border-color: var(--red); color: var(--red); }}
        .ext-link.primary {{ background: var(--red); color: white; border-color: var(--red); }}
    </style>
</head>
<body>
    <a href="javascript:void(0)" onclick="window.location.href='marshall://osint'" class="back-link">&#8592; Back to OSINT Tools</a>
    <div class="header">
        <h1>OSINT Report</h1>
        <div class="target">{domain}</div>
    </div>
    <div class="container">
        <div class="report-section">
            <div class="section-header">Domain Information</div>
            <div class="section-content">
                <p>Target: <strong>{domain}</strong></p>
                <p style="color: var(--green); margin-top: 0.5rem;">&#9679; Analysis Ready</p>
                <div class="external-links">
                    <a href="https://who.is/{domain}" target="_blank" class="ext-link">WHOIS Lookup</a>
                    <a href="https://dnsdumpster.com/?search={domain}" target="_blank" class="ext-link">DNS Records</a>
                    <a href="https://www.shodan.io/search?query={domain}" target="_blank" class="ext-link">Shodan</a>
                    <a href="https://crt.sh/?q={domain}" target="_blank" class="ext-link">Certificates</a>
                    <a href="https://www.ssllabs.com/ssltest/analyze.html?d={domain}" target="_blank" class="ext-link">SSL Test</a>
                </div>
            </div>
        </div>
        <div class="report-section">
            <div class="section-header">Quick Actions</div>
            <div class="section-content">
                <p style="color: var(--fg-dim); margin-bottom: 1rem;">Click any tool above to perform detailed analysis.</p>
                <a href="javascript:void(0)" onclick="window.location.href='marshall://assistant'" class="ext-link primary">Ask Dr Marshall about this target</a>
            </div>
        </div>
    </div>
</body>
</html>"##, domain = domain)
}

/// Generate Menu/Dashboard page with settings links
pub fn generate_menu_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Marshall Menu</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }
        .menu-container {
            max-width: 400px;
            margin: 0 auto;
            padding: 2rem;
        }
        .menu-header {
            text-align: center;
            margin-bottom: 2rem;
            padding-bottom: 1rem;
            border-bottom: 2px solid var(--red);
        }
        .menu-header h1 {
            color: var(--red);
            font-size: 24px;
            letter-spacing: 4px;
        }
        .menu-header p {
            color: var(--fg-dim);
            font-size: 12px;
            margin-top: 0.5rem;
        }
        .menu-list {
            list-style: none;
        }
        .menu-item {
            margin-bottom: 8px;
        }
        .menu-link {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 14px 18px;
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            text-decoration: none;
            transition: all 0.2s ease;
            cursor: pointer;
        }
        .menu-link:hover {
            border-color: var(--red);
            background: var(--bg3);
        }
        .menu-icon {
            font-size: 20px;
            width: 28px;
            text-align: center;
        }
        .menu-text {
            flex: 1;
        }
        .menu-text strong {
            display: block;
            margin-bottom: 2px;
        }
        .menu-text small {
            color: var(--fg-dim);
            font-size: 11px;
        }
        .menu-divider {
            height: 1px;
            background: var(--border);
            margin: 1rem 0;
        }
        .version {
            text-align: center;
            color: var(--fg-dim);
            font-size: 11px;
            margin-top: 2rem;
        }
    </style>
</head>
<body>
    <div class="menu-container">
        <div class="menu-header">
            <h1>MARSHALL</h1>
            <p>NullSec Private Browser</p>
        </div>
        <ul class="menu-list">
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://home'" class="menu-link">
                    <span class="menu-icon">&#127968;</span>
                    <span class="menu-text">
                        <strong>Home</strong>
                        <small>Return to homepage</small>
                    </span>
                </a>
            </li>
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://assistant'" class="menu-link">
                    <span class="menu-icon">&#129302;</span>
                    <span class="menu-text">
                        <strong>Dr Marshall AI</strong>
                        <small>Private AI assistant</small>
                    </span>
                </a>
            </li>
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://osint'" class="menu-link">
                    <span class="menu-icon">&#128269;</span>
                    <span class="menu-text">
                        <strong>OSINT Tools</strong>
                        <small>Intelligence gathering</small>
                    </span>
                </a>
            </li>
            </li>
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://workforce'" class="menu-link">
                    <span class="menu-icon">&#128101;</span>
                    <span class="menu-text">
                        <strong>Workforce Center</strong>
                        <small>Employee management</small>
                    </span>
                </a>
            </li>
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://voip'" class="menu-link">
                    <span class="menu-icon">&#128222;</span>
                    <span class="menu-text">
                        <strong>VoIP Center</strong>
                        <small>Secure communications</small>
                    </span>
                </a>
            <div class="menu-divider"></div>
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://settings'" class="menu-link">
                    <span class="menu-icon">&#9881;</span>
                    <span class="menu-text">
                        <strong>Settings</strong>
                        <small>Configure browser options</small>
                    </span>
                </a>
            </li>
            <li class="menu-item">
                <a href="javascript:void(0)" onclick="window.location.href='marshall://privacy'" class="menu-link">
                    <span class="menu-icon">&#128274;</span>
                    <span class="menu-text">
                        <strong>Privacy</strong>
                        <small>Protection status</small>
                    </span>
                </a>
            </li>
        </ul>
        <div class="version">
            Marshall v2.0.0 | <span style="color: var(--green);">&#9679;</span> Secure
        </div>
    </div>
</body>
</html>"##.to_string()
}

/// Marshall userscript - rebrands DuckDuckGo to Marshall
pub fn generate_userscript() -> String {
    r##"(function() {
    'use strict';
    
    // Marshall branding - minimal header with logo only
    const MARSHALL_LOGO = `
        <div id="marshall-header" style="
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 999999;
            background: linear-gradient(180deg, #0d0d0d 0%, rgba(13,13,13,0.95) 100%);
            padding: 8px 20px;
            display: flex;
            align-items: center;
            gap: 12px;
            border-bottom: 2px solid #ff0040;
            font-family: 'Segoe UI', -apple-system, sans-serif;
        ">
            <svg viewBox="0 0 100 100" width="32" height="32" fill="none">
                <circle cx="50" cy="40" r="24" fill="#0d0d0d" stroke="#ff0040" stroke-width="2"/>
                <path d="M50 38 L70 48 L50 53 Z" fill="#ff0040"/>
                <circle cx="40" cy="36" r="4" fill="#00ff88"/>
                <circle cx="60" cy="36" r="4" fill="#00ff88"/>
            </svg>
            <span style="color: #ff0040; font-weight: 700; font-size: 18px; letter-spacing: 3px;">MARSHALL</span>
        </div>
    `;
    
    // CSS to hide ALL DDG elements except search bar and results
    const HIDE_DDG_CSS = `
        /* Hide ALL DDG branding, buttons, links, headers, navs */
        .logo-wrap, .logo_homepage, .ddg-logo, 
        [class*="Logo"], [class*="logo"],
        .header__logo, .js-logo-link,
        a[href="/"] > svg, .badge-link,
        .feedback-btn, .js-feedback-btn,
        [data-testid="logo"],
        .header__button, .header-wrap,
        .nav-link, .dropdown, .dropdown--settings,
        .js-side-menu-open, .js-side-menu-close,
        .modal, .modal-wrapper,
        .badge, .badge-link, 
        .serp__top-right, .header__icons,
        [class*="dropdown"], [class*="Dropdown"],
        [class*="modal"], [class*="Modal"],
        [class*="sidebar"], [class*="Sidebar"],
        .zcm__link, .zcm__wrap, .zci__more-link,
        .module--carousel, .module--about,
        .is-not-mobile-device .header__button,
        .footer, [class*="Footer"],
        .about-profiles, .module--people,
        .related-searches, .js-about-module,
        .module--news__btn, .result__menu,
        .result__icon, .tile--img,
        .tile--vid, .module--images,
        .module--videos, .module--news,
        .vertical--news, .vertical--video,
        .vertical--image, .zci,
        .search__button, .search__clear,
        .header, nav, header {
            display: none !important;
            visibility: hidden !important;
            opacity: 0 !important;
            pointer-events: none !important;
        }
        
        /* Keep and show only the search input */
        .search, .search__input, .searchbox_input__bEGm3,
        input[type="text"], input[name="q"],
        .search-form, .search__form {
            display: flex !important;
            visibility: visible !important;
            opacity: 1 !important;
            pointer-events: auto !important;
        }
        
        /* Adjust body for Marshall header */
        body { 
            padding-top: 52px !important; 
            background: #0d0d0d !important;
        }
        
        /* Style search results dark */
        .result, .results, .result__body,
        .results--main, .serp__results,
        [data-testid="result"] {
            background: #1a1a1a !important;
            border-color: #333 !important;
            display: block !important;
            visibility: visible !important;
            opacity: 1 !important;
        }
        
        .result__a { color: #ff0040 !important; }
        .result__snippet { color: #e0e0e0 !important; }
        .result__url { color: #00ff88 !important; }
        
        /* Clean minimal search bar style */
        .search-form, #search_form, .searchbox {
            background: #1a1a1a !important;
            border: 1px solid #333 !important;
            border-radius: 8px !important;
            margin: 0 auto !important;
            max-width: 600px !important;
        }
        
        input[name="q"], .searchbox_input__bEGm3 {
            background: transparent !important;
            color: #e0e0e0 !important;
            border: none !important;
        }
    `;
    
    function injectMarshall() {
        // Add CSS
        if (!document.getElementById('marshall-css')) {
            const style = document.createElement('style');
            style.id = 'marshall-css';
            style.textContent = HIDE_DDG_CSS;
            document.head.appendChild(style);
        }
        
        // Add header
        if (!document.getElementById('marshall-header')) {
            document.body.insertAdjacentHTML('afterbegin', MARSHALL_LOGO);
        }
        
        // Replace any remaining DDG text
        document.querySelectorAll('*').forEach(el => {
            if (el.childNodes.length === 1 && el.childNodes[0].nodeType === 3) {
                const text = el.textContent;
                if (text && (text.includes('DuckDuckGo') || text.includes('Duck.ai'))) {
                    el.textContent = text
                        .replace(/DuckDuckGo/gi, 'Marshall')
                        .replace(/Duck\.ai/gi, 'Dr Marshall')
                        .replace(/Duck AI/gi, 'Dr Marshall');
                }
            }
        });
    }
    
    // Redirect duck.ai to Dr Marshall
    if (window.location.hostname === 'duck.ai' || window.location.href.includes('duck.ai')) {
        window.location.href = 'marshall://assistant';
        return;
    }
    
    // Run on DuckDuckGo
    if (window.location.hostname.includes('duckduckgo')) {
        injectMarshall();
        
        // Keep running for dynamic content
        const observer = new MutationObserver(injectMarshall);
        observer.observe(document.body, { childList: true, subtree: true });
        
        setInterval(injectMarshall, 1000);
    }
    
    console.log('[Marshall] Branding injected');
})();"##.to_string()
}

/// Generate Workforce Management page
pub fn generate_workforce_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Workforce Center - Marshall</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --yellow: #ffcc00;
            --blue: #00a8ff;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }
        a { color: var(--red); text-decoration: none; }
        a:hover { text-decoration: underline; }
        .back-link {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color: var(--fg-dim);
            font-size: 14px;
            margin: 1rem 2rem;
        }
        .header {
            background: linear-gradient(135deg, var(--bg2) 0%, var(--bg) 100%);
            border-bottom: 2px solid var(--green);
            padding: 1.5rem 2rem;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .header-left { display: flex; align-items: center; gap: 1rem; }
        .header h1 { font-size: 24px; color: var(--green); }
        .header p { color: var(--fg-dim); font-size: 13px; }
        .header-actions { display: flex; gap: 12px; }
        .btn {
            padding: 10px 20px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 13px;
            cursor: pointer;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .btn:hover { border-color: var(--green); color: var(--green); }
        .btn.primary { background: var(--green); color: #000; border: none; font-weight: 600; }
        .btn.primary:hover { background: #00cc6a; }
        .dashboard {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 1rem;
            padding: 1.5rem 2rem;
        }
        .stat-card {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 12px;
            padding: 1.25rem;
        }
        .stat-label { color: var(--fg-dim); font-size: 12px; margin-bottom: 8px; }
        .stat-value { font-size: 28px; font-weight: 700; color: var(--green); }
        .stat-value.warning { color: var(--yellow); }
        .stat-sub { font-size: 11px; color: var(--fg-dim); margin-top: 4px; }
        .section {
            padding: 0 2rem 2rem;
        }
        .section-title {
            font-size: 16px;
            color: var(--fg);
            margin-bottom: 1rem;
            display: flex;
            align-items: center;
            gap: 8px;
        }
        .table-container {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 12px;
            overflow: hidden;
        }
        table {
            width: 100%;
            border-collapse: collapse;
        }
        th, td {
            padding: 12px 16px;
            text-align: left;
            border-bottom: 1px solid var(--border);
        }
        th {
            background: var(--bg3);
            font-size: 12px;
            font-weight: 600;
            color: var(--fg-dim);
            text-transform: uppercase;
        }
        tr:last-child td { border-bottom: none; }
        tr:hover td { background: var(--bg3); }
        .status-badge {
            padding: 4px 10px;
            border-radius: 12px;
            font-size: 11px;
            font-weight: 600;
        }
        .status-active { background: rgba(0,255,136,0.15); color: var(--green); }
        .status-offline { background: rgba(128,128,128,0.15); color: var(--fg-dim); }
        .status-clocked { background: rgba(0,168,255,0.15); color: var(--blue); }
        .avatar {
            width: 32px;
            height: 32px;
            background: var(--bg3);
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 14px;
        }
        .employee-info { display: flex; align-items: center; gap: 12px; }
        .employee-name { font-weight: 600; }
        .employee-role { font-size: 12px; color: var(--fg-dim); }
        .clock-btn {
            padding: 6px 12px;
            border-radius: 6px;
            font-size: 11px;
            border: none;
            cursor: pointer;
        }
        .clock-in { background: var(--green); color: #000; }
        .clock-out { background: var(--red); color: white; }
        .modal-overlay {
            position: fixed;
            top: 0; left: 0; right: 0; bottom: 0;
            background: rgba(0,0,0,0.8);
            display: none;
            align-items: center;
            justify-content: center;
            z-index: 1000;
        }
        .modal {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 16px;
            width: 90%;
            max-width: 500px;
            padding: 1.5rem;
        }
        .modal h2 { color: var(--green); margin-bottom: 1rem; }
        .form-group { margin-bottom: 1rem; }
        .form-group label { display: block; font-size: 13px; margin-bottom: 6px; color: var(--fg-dim); }
        .form-group input, .form-group select {
            width: 100%;
            padding: 12px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 14px;
        }
        .form-group input:focus, .form-group select:focus { border-color: var(--green); outline: none; }
        .modal-actions { display: flex; gap: 12px; justify-content: flex-end; margin-top: 1.5rem; }
        .tabs { display: flex; gap: 4px; padding: 0 2rem; margin-bottom: 1rem; }
        .tab {
            padding: 10px 20px;
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 8px 8px 0 0;
            color: var(--fg-dim);
            cursor: pointer;
            font-size: 13px;
        }
        .tab.active { background: var(--bg3); color: var(--green); border-color: var(--green); border-bottom-color: var(--bg3); }
    </style>
</head>
<body>
    <a href="javascript:void(0)" onclick="window.location.href='marshall://home'" class="back-link">← Back to Home</a>
    
    <div class="header">
        <div class="header-left">
            <div>
                <h1>👥 Workforce Center</h1>
                <p>Employee & Project Management</p>
            </div>
        </div>
        <div class="header-actions">
            <button class="btn" onclick="showModal('timecard')">⏱️ Time Cards</button>
            <button class="btn" onclick="showModal('project')">📁 Projects</button>
            <button class="btn primary" onclick="showModal('employee')">+ Add Employee</button>
        </div>
    </div>
    
    <div class="dashboard">
        <div class="stat-card">
            <div class="stat-label">Total Employees</div>
            <div class="stat-value" id="totalEmployees">5</div>
            <div class="stat-sub">3 active today</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Clocked In</div>
            <div class="stat-value" id="clockedIn">2</div>
            <div class="stat-sub">Currently working</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Hours Today</div>
            <div class="stat-value" id="hoursToday">14.5</div>
            <div class="stat-sub">Team total</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Active Projects</div>
            <div class="stat-value" id="activeProjects">3</div>
            <div class="stat-sub">2 due this week</div>
        </div>
        <div class="stat-card">
            <div class="stat-label">Pending Payouts</div>
            <div class="stat-value warning" id="pendingPayouts">$2,450</div>
            <div class="stat-sub">4 employees</div>
        </div>
    </div>
    
    <div class="tabs">
        <div class="tab active" onclick="showTab('employees')">Employees</div>
        <div class="tab" onclick="showTab('timecards')">Time Cards</div>
        <div class="tab" onclick="showTab('projects')">Projects</div>
        <div class="tab" onclick="showTab('payouts')">Payouts</div>
    </div>
    
    <div class="section" id="employees-section">
        <div class="table-container">
            <table>
                <thead>
                    <tr>
                        <th>Employee</th>
                        <th>Status</th>
                        <th>Project</th>
                        <th>Hours (Week)</th>
                        <th>Rate</th>
                        <th>Actions</th>
                    </tr>
                </thead>
                <tbody id="employeeTable">
                    <tr>
                        <td>
                            <div class="employee-info">
                                <div class="avatar">👤</div>
                                <div>
                                    <div class="employee-name">Alex Thompson</div>
                                    <div class="employee-role">Senior Developer</div>
                                </div>
                            </div>
                        </td>
                        <td><span class="status-badge status-clocked">Clocked In</span></td>
                        <td>Project Alpha</td>
                        <td>32.5h</td>
                        <td>$45/hr</td>
                        <td><button class="clock-btn clock-out" onclick="clockOut(1)">Clock Out</button></td>
                    </tr>
                    <tr>
                        <td>
                            <div class="employee-info">
                                <div class="avatar">👤</div>
                                <div>
                                    <div class="employee-name">Sarah Chen</div>
                                    <div class="employee-role">UI Designer</div>
                                </div>
                            </div>
                        </td>
                        <td><span class="status-badge status-clocked">Clocked In</span></td>
                        <td>Marshall UI</td>
                        <td>28.0h</td>
                        <td>$40/hr</td>
                        <td><button class="clock-btn clock-out" onclick="clockOut(2)">Clock Out</button></td>
                    </tr>
                    <tr>
                        <td>
                            <div class="employee-info">
                                <div class="avatar">👤</div>
                                <div>
                                    <div class="employee-name">Mike Johnson</div>
                                    <div class="employee-role">Security Analyst</div>
                                </div>
                            </div>
                        </td>
                        <td><span class="status-badge status-active">Active</span></td>
                        <td>OSINT Tools</td>
                        <td>40.0h</td>
                        <td>$50/hr</td>
                        <td><button class="clock-btn clock-in" onclick="clockIn(3)">Clock In</button></td>
                    </tr>
                </tbody>
            </table>
        </div>
    </div>
    
    <!-- Add Employee Modal -->
    <div class="modal-overlay" id="employeeModal">
        <div class="modal">
            <h2>➕ Add New Employee</h2>
            <div class="form-group">
                <label>Full Name</label>
                <input type="text" id="empName" placeholder="Enter full name">
            </div>
            <div class="form-group">
                <label>Role / Position</label>
                <input type="text" id="empRole" placeholder="e.g., Developer, Designer">
            </div>
            <div class="form-group">
                <label>Email</label>
                <input type="email" id="empEmail" placeholder="employee@example.com">
            </div>
            <div class="form-group">
                <label>Hourly Rate ($)</label>
                <input type="number" id="empRate" placeholder="25.00" step="0.01">
            </div>
            <div class="form-group">
                <label>Assigned Project</label>
                <select id="empProject">
                    <option value="">Select project...</option>
                    <option value="alpha">Project Alpha</option>
                    <option value="marshall">Marshall UI</option>
                    <option value="osint">OSINT Tools</option>
                </select>
            </div>
            <div class="modal-actions">
                <button class="btn" onclick="hideModal('employee')">Cancel</button>
                <button class="btn primary" onclick="addEmployee()">Add Employee</button>
            </div>
        </div>
    </div>
    
    <script>
        function showModal(type) {
            document.getElementById(type + 'Modal').style.display = 'flex';
        }
        function hideModal(type) {
            document.getElementById(type + 'Modal').style.display = 'none';
        }
        function showTab(tab) {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            event.target.classList.add('active');
        }
        function clockIn(id) {
            alert('Clock In - Employee #' + id + '\nConnecting to workforce backend...');
        }
        function clockOut(id) {
            alert('Clock Out - Employee #' + id + '\nTime recorded.');
        }
        function addEmployee() {
            var name = document.getElementById('empName').value;
            if (name) {
                alert('Employee "' + name + '" added successfully!');
                hideModal('employee');
            }
        }
    </script>
</body>
</html>"##.to_string()
}

/// Generate VoIP Communications page
pub fn generate_voip_page() -> String {
    r##"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>VoIP Center - Marshall</title>
    <style>
        * { box-sizing: border-box; margin: 0; padding: 0; }
        :root {
            --red: #ff0040;
            --green: #00ff88;
            --yellow: #ffcc00;
            --blue: #00a8ff;
            --purple: #a855f7;
            --bg: #0d0d0d;
            --bg2: #1a1a1a;
            --bg3: #252525;
            --fg: #e0e0e0;
            --fg-dim: #808080;
            --border: #333;
        }
        html, body {
            background: var(--bg);
            color: var(--fg);
            font-family: 'Segoe UI', -apple-system, sans-serif;
            min-height: 100vh;
        }
        a { color: var(--red); text-decoration: none; }
        .back-link {
            display: inline-flex;
            align-items: center;
            gap: 8px;
            color: var(--fg-dim);
            font-size: 14px;
            margin: 1rem 2rem;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 2rem;
        }
        .header {
            background: linear-gradient(135deg, var(--bg2) 0%, var(--bg) 100%);
            border-bottom: 2px solid var(--purple);
            padding: 1.5rem 2rem;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .header h1 { font-size: 24px; color: var(--purple); display: flex; align-items: center; gap: 12px; }
        .header p { color: var(--fg-dim); font-size: 13px; }
        .status-online { color: var(--green); display: flex; align-items: center; gap: 8px; }
        .status-dot { width: 10px; height: 10px; background: var(--green); border-radius: 50%; animation: pulse 2s infinite; }
        @keyframes pulse { 0%, 100% { opacity: 1; } 50% { opacity: 0.5; } }
        .main-layout {
            display: grid;
            grid-template-columns: 300px 1fr 280px;
            gap: 1.5rem;
            padding: 1.5rem 2rem;
            min-height: calc(100vh - 120px);
        }
        .panel {
            background: var(--bg2);
            border: 1px solid var(--border);
            border-radius: 12px;
            overflow: hidden;
        }
        .panel-header {
            padding: 1rem;
            background: var(--bg3);
            border-bottom: 1px solid var(--border);
            font-weight: 600;
            font-size: 14px;
            display: flex;
            align-items: center;
            justify-content: space-between;
        }
        .panel-body { padding: 1rem; }
        
        /* Dialpad */
        .dialpad {
            display: grid;
            grid-template-columns: repeat(3, 1fr);
            gap: 10px;
            margin-bottom: 1rem;
        }
        .dial-btn {
            padding: 18px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 12px;
            color: var(--fg);
            font-size: 24px;
            cursor: pointer;
            transition: all 0.15s;
        }
        .dial-btn:hover { background: var(--bg); border-color: var(--purple); }
        .dial-btn:active { transform: scale(0.95); }
        .dial-btn small { display: block; font-size: 9px; color: var(--fg-dim); margin-top: 2px; }
        .phone-input {
            width: 100%;
            padding: 14px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 8px;
            color: var(--fg);
            font-size: 20px;
            text-align: center;
            letter-spacing: 2px;
            margin-bottom: 1rem;
        }
        .call-actions {
            display: flex;
            gap: 12px;
            justify-content: center;
        }
        .call-btn {
            width: 60px;
            height: 60px;
            border-radius: 50%;
            border: none;
            cursor: pointer;
            font-size: 24px;
            transition: transform 0.2s;
        }
        .call-btn:hover { transform: scale(1.1); }
        .call-btn.green { background: var(--green); }
        .call-btn.red { background: var(--red); }
        
        /* Call Display */
        .call-display {
            text-align: center;
            padding: 2rem;
        }
        .call-avatar {
            width: 120px;
            height: 120px;
            background: var(--bg3);
            border-radius: 50%;
            margin: 0 auto 1.5rem;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 48px;
            border: 3px solid var(--border);
        }
        .call-avatar.active { border-color: var(--green); animation: ring 1s infinite; }
        @keyframes ring { 0%, 100% { box-shadow: 0 0 0 0 rgba(0,255,136,0.4); } 50% { box-shadow: 0 0 0 20px rgba(0,255,136,0); } }
        .call-name { font-size: 24px; font-weight: 600; margin-bottom: 4px; }
        .call-number { color: var(--fg-dim); font-size: 14px; margin-bottom: 8px; }
        .call-status { color: var(--green); font-size: 18px; margin-bottom: 2rem; }
        .call-status.ringing { color: var(--yellow); }
        .call-timer { font-size: 32px; font-weight: 300; color: var(--fg); margin-bottom: 2rem; font-family: monospace; }
        .call-controls {
            display: flex;
            gap: 16px;
            justify-content: center;
            flex-wrap: wrap;
        }
        .ctrl-btn {
            width: 56px;
            height: 56px;
            background: var(--bg3);
            border: 1px solid var(--border);
            border-radius: 12px;
            color: var(--fg);
            font-size: 20px;
            cursor: pointer;
            display: flex;
            flex-direction: column;
            align-items: center;
            justify-content: center;
            gap: 4px;
        }
        .ctrl-btn span { font-size: 9px; }
        .ctrl-btn:hover { border-color: var(--purple); }
        .ctrl-btn.active { background: var(--purple); border-color: var(--purple); }
        .ctrl-btn.hangup { background: var(--red); border-color: var(--red); }
        
        /* Contacts / Recent */
        .contact-list { max-height: 400px; overflow-y: auto; }
        .contact-item {
            display: flex;
            align-items: center;
            gap: 12px;
            padding: 12px;
            border-radius: 8px;
            cursor: pointer;
            transition: background 0.2s;
        }
        .contact-item:hover { background: var(--bg3); }
        .contact-avatar {
            width: 40px;
            height: 40px;
            background: var(--bg3);
            border-radius: 50%;
            display: flex;
            align-items: center;
            justify-content: center;
            font-size: 16px;
        }
        .contact-info { flex: 1; }
        .contact-name { font-weight: 600; font-size: 14px; }
        .contact-number { font-size: 12px; color: var(--fg-dim); }
        .contact-action {
            width: 36px;
            height: 36px;
            background: var(--green);
            border: none;
            border-radius: 50%;
            cursor: pointer;
            font-size: 14px;
        }
        
        /* Recent Calls */
        .recent-item { padding: 10px 12px; }
        .recent-type { font-size: 14px; margin-right: 8px; }
        .recent-type.incoming { color: var(--green); }
        .recent-type.outgoing { color: var(--blue); }
        .recent-type.missed { color: var(--red); }
        .recent-time { font-size: 11px; color: var(--fg-dim); }
        
        /* Settings Panel */
        .setting-row {
            display: flex;
            align-items: center;
            justify-content: space-between;
            padding: 12px 0;
            border-bottom: 1px solid var(--border);
        }
        .setting-row:last-child { border-bottom: none; }
        .setting-label { font-size: 13px; }
        .setting-value { font-size: 13px; color: var(--fg-dim); }
        
        .no-call-message {
            text-align: center;
            padding: 3rem;
            color: var(--fg-dim);
        }
        .no-call-message .icon { font-size: 64px; margin-bottom: 1rem; opacity: 0.5; }
    </style>
</head>
<body>
    <a href="javascript:void(0)" onclick="window.location.href='marshall://home'" class="back-link">← Back to Home</a>
    
    <div class="header">
        <div>
            <h1>📞 VoIP Center</h1>
            <p>Secure Communications Hub</p>
        </div>
        <div class="status-online">
            <div class="status-dot"></div>
            <span>Connected • SIP Ready</span>
        </div>
    </div>
    
    <div class="main-layout">
        <!-- Left Panel: Dialpad & Contacts -->
        <div>
            <div class="panel" style="margin-bottom: 1rem;">
                <div class="panel-header">Dialpad</div>
                <div class="panel-body">
                    <input type="tel" class="phone-input" id="phoneNumber" placeholder="+1 (555) 000-0000" oninput="formatPhone(this)">
                    <div class="dialpad">
                        <button class="dial-btn" onclick="dialNum('1')">1<small></small></button>
                        <button class="dial-btn" onclick="dialNum('2')">2<small>ABC</small></button>
                        <button class="dial-btn" onclick="dialNum('3')">3<small>DEF</small></button>
                        <button class="dial-btn" onclick="dialNum('4')">4<small>GHI</small></button>
                        <button class="dial-btn" onclick="dialNum('5')">5<small>JKL</small></button>
                        <button class="dial-btn" onclick="dialNum('6')">6<small>MNO</small></button>
                        <button class="dial-btn" onclick="dialNum('7')">7<small>PQRS</small></button>
                        <button class="dial-btn" onclick="dialNum('8')">8<small>TUV</small></button>
                        <button class="dial-btn" onclick="dialNum('9')">9<small>WXYZ</small></button>
                        <button class="dial-btn" onclick="dialNum('*')">*</button>
                        <button class="dial-btn" onclick="dialNum('0')">0<small>+</small></button>
                        <button class="dial-btn" onclick="dialNum('#')">#</button>
                    </div>
                    <div class="call-actions">
                        <button class="call-btn green" onclick="makeCall()">📞</button>
                        <button class="call-btn red" onclick="clearNumber()">⌫</button>
                    </div>
                </div>
            </div>
            
            <div class="panel">
                <div class="panel-header">Quick Contacts</div>
                <div class="panel-body contact-list">
                    <div class="contact-item" onclick="callContact('Support', '+1-800-555-0100')">
                        <div class="contact-avatar">🎧</div>
                        <div class="contact-info">
                            <div class="contact-name">Support Line</div>
                            <div class="contact-number">+1-800-555-0100</div>
                        </div>
                        <button class="contact-action">📞</button>
                    </div>
                    <div class="contact-item" onclick="callContact('Office', '+1-555-123-4567')">
                        <div class="contact-avatar">🏢</div>
                        <div class="contact-info">
                            <div class="contact-name">Main Office</div>
                            <div class="contact-number">+1-555-123-4567</div>
                        </div>
                        <button class="contact-action">📞</button>
                    </div>
                    <div class="contact-item" onclick="callContact('Emergency', '+1-555-911-0000')">
                        <div class="contact-avatar">🚨</div>
                        <div class="contact-info">
                            <div class="contact-name">Emergency</div>
                            <div class="contact-number">+1-555-911-0000</div>
                        </div>
                        <button class="contact-action">📞</button>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Center: Call Display -->
        <div class="panel">
            <div class="panel-header">
                <span>Active Call</span>
                <span id="callQuality" style="color: var(--green);">● HD</span>
            </div>
            <div class="panel-body">
                <div class="call-display" id="callDisplay">
                    <div class="no-call-message">
                        <div class="icon">📞</div>
                        <h3>No Active Call</h3>
                        <p>Use the dialpad or select a contact to start a call</p>
                    </div>
                </div>
            </div>
        </div>
        
        <!-- Right: Recent & Settings -->
        <div>
            <div class="panel" style="margin-bottom: 1rem;">
                <div class="panel-header">Recent Calls</div>
                <div class="panel-body contact-list">
                    <div class="contact-item recent-item">
                        <span class="recent-type outgoing">↗</span>
                        <div class="contact-info">
                            <div class="contact-name">Support Line</div>
                            <div class="contact-number recent-time">Today, 2:34 PM • 5:23</div>
                        </div>
                    </div>
                    <div class="contact-item recent-item">
                        <span class="recent-type incoming">↙</span>
                        <div class="contact-info">
                            <div class="contact-name">+1-555-987-6543</div>
                            <div class="contact-number recent-time">Today, 11:20 AM • 12:45</div>
                        </div>
                    </div>
                    <div class="contact-item recent-item">
                        <span class="recent-type missed">✕</span>
                        <div class="contact-info">
                            <div class="contact-name">Unknown</div>
                            <div class="contact-number recent-time">Yesterday, 4:15 PM</div>
                        </div>
                    </div>
                </div>
            </div>
            
            <div class="panel">
                <div class="panel-header">SIP Settings</div>
                <div class="panel-body">
                    <div class="setting-row">
                        <span class="setting-label">Server</span>
                        <span class="setting-value">sip.nullsec.io</span>
                    </div>
                    <div class="setting-row">
                        <span class="setting-label">Username</span>
                        <span class="setting-value">operator@nullsec</span>
                    </div>
                    <div class="setting-row">
                        <span class="setting-label">Codec</span>
                        <span class="setting-value">OPUS / G.722</span>
                    </div>
                    <div class="setting-row">
                        <span class="setting-label">Encryption</span>
                        <span class="setting-value" style="color: var(--green);">SRTP Enabled</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
    
    <script>
        var callActive = false;
        var callTimer = null;
        var callSeconds = 0;
        
        function dialNum(num) {
            var input = document.getElementById('phoneNumber');
            input.value += num;
        }
        
        function clearNumber() {
            var input = document.getElementById('phoneNumber');
            input.value = input.value.slice(0, -1);
        }
        
        function formatPhone(input) {
            var val = input.value.replace(/\D/g, '');
            if (val.length > 10) val = val.slice(0, 11);
            input.value = val;
        }
        
        function makeCall() {
            var number = document.getElementById('phoneNumber').value;
            if (!number) return alert('Enter a phone number');
            startCall('Dialing...', number);
        }
        
        function callContact(name, number) {
            startCall(name, number);
        }
        
        function startCall(name, number) {
            callActive = true;
            callSeconds = 0;
            
            document.getElementById('callDisplay').innerHTML = 
                '<div class="call-avatar active">📞</div>' +
                '<div class="call-name">' + name + '</div>' +
                '<div class="call-number">' + number + '</div>' +
                '<div class="call-status ringing">Connecting...</div>' +
                '<div class="call-timer" id="timer">00:00</div>' +
                '<div class="call-controls">' +
                    '<button class="ctrl-btn" onclick="toggleMute(this)">🔇<span>Mute</span></button>' +
                    '<button class="ctrl-btn" onclick="toggleHold(this)">⏸<span>Hold</span></button>' +
                    '<button class="ctrl-btn" onclick="toggleSpeaker(this)">🔊<span>Speaker</span></button>' +
                    '<button class="ctrl-btn" onclick="showKeypad()">⌨<span>Keypad</span></button>' +
                    '<button class="ctrl-btn hangup" onclick="endCall()">📵<span>End</span></button>' +
                '</div>';
            
            // Simulate connection
            setTimeout(function() {
                if (callActive) {
                    document.querySelector('.call-status').textContent = 'Connected';
                    document.querySelector('.call-status').classList.remove('ringing');
                    callTimer = setInterval(updateTimer, 1000);
                }
            }, 2000);
        }
        
        function updateTimer() {
            callSeconds++;
            var mins = Math.floor(callSeconds / 60).toString().padStart(2, '0');
            var secs = (callSeconds % 60).toString().padStart(2, '0');
            document.getElementById('timer').textContent = mins + ':' + secs;
        }
        
        function endCall() {
            callActive = false;
            clearInterval(callTimer);
            document.getElementById('callDisplay').innerHTML = 
                '<div class="no-call-message">' +
                    '<div class="icon">📞</div>' +
                    '<h3>Call Ended</h3>' +
                    '<p>Duration: ' + document.getElementById('timer')?.textContent + '</p>' +
                '</div>';
        }
        
        function toggleMute(btn) { btn.classList.toggle('active'); }
        function toggleHold(btn) { btn.classList.toggle('active'); }
        function toggleSpeaker(btn) { btn.classList.toggle('active'); }
        function showKeypad() { alert('DTMF Keypad - Send tones during call'); }
    </script>
</body>
</html>"##.to_string()
}
