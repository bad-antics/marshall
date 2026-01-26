# Configuration

## Config File Location
Marshall stores configuration in `~/.config/marshall/config.toml`

## Default Configuration
```toml
[general]
homepage = "https://duckduckgo.com"
search_engine = "duckduckgo"
restore_session = true

[appearance]
theme = "dark"
font_family = "Sans"
font_size = 16
zoom_level = 1.0

[privacy]
enable_javascript = true
enable_cookies = true
block_trackers = true
do_not_track = true
clear_on_exit = false

[ai]
enabled = true
provider = "ollama"
model = "llama3"
endpoint = "http://localhost:11434"

[downloads]
directory = "~/Downloads"
ask_location = false

[developer]
show_inspector = false
enable_webgl = true
hardware_acceleration = true
```

## Environment Variables
| Variable | Description |
|----------|-------------|
| `MARSHALL_CONFIG` | Custom config path |
| `MARSHALL_DATA` | Data directory |
| `MARSHALL_CACHE` | Cache directory |

## Search Engines
Supported search engines:
- `duckduckgo` (default)
- `google`
- `brave`
- `startpage`
- `searx`

## AI Providers
Supported AI backends:
- `ollama` - Local Ollama instance
- `openai` - OpenAI API
- `anthropic` - Anthropic Claude
