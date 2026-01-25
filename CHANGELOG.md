# Changelog

All notable changes to Marshall will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.1.0] - 2026-01-25

### Added
- **Dr. Marshall AI Assistant** - Grok-level conversational AI with multi-provider support
  - Support for 10+ AI providers: OpenAI, Anthropic, Groq, xAI (Grok), Ollama, LMStudio, OpenRouter, Together AI, Mistral, Cohere
  - Comprehensive Dr. Marshall system prompt for security-focused intelligence
  - Function calling support for browser actions, OSINT, VoIP, and Workforce tools
  - GTK4-based chat interface with cyberpunk styling
  - Local AI support via Ollama for privacy-conscious users
  - Automatic fallback to pattern matching when AI unavailable

- **AI Features**
  - Context-aware conversations with full history
  - Tool integration: OSINT scans, web search, navigation, VoIP calls, workforce actions
  - Configurable temperature, max tokens, and other model parameters
  - API key management for cloud providers
  - Model switching between local and cloud providers

- **Chat Interface**
  - Modern cyberpunk-themed GTK4 chat panel
  - Real-time message streaming
  - Markdown rendering in responses
  - Settings dialog for AI configuration
  - Welcome message with capability overview

### Changed
- Upgraded assistant module with AI engine integration
- Updated conversation manager with AI fallback
- Improved response parsing for action extraction
- Bumped version to 2.1.0

### Fixed
- Assistant state management improvements
- Better error handling for AI API failures

## [2.0.0] - 2026-01-24

### Added
- **Workforce Management System**
  - Employee database with profiles
  - Time clock functionality (clock in/out)
  - Timecard management and reporting
  - Project tracking and task assignment
  - Performance metrics dashboard
  - Role-based access control (Admin, Manager, Employee)

- **VoIP Communication Suite**
  - SIP/WebRTC calling capabilities
  - Contact management with avatars
  - Call history with duration tracking
  - Voicemail support
  - Conference calling
  - Audio settings (input/output device selection)

- **AI Model Selection**
  - Support for multiple AI providers
  - Configurable model parameters
  - Local and cloud model options

- **Enhanced UI**
  - New Workforce page with employee management
  - VoIP page with dialer and contacts
  - Improved dashboard with quick actions
  - Cyberpunk-themed interface

### Changed
- Updated main navigation with Workforce and VoIP tabs
- Improved search with OSINT integration
- Enhanced privacy controls

### Security
- Added VoIP encryption options
- Improved data isolation for workforce features

## [1.0.0] - 2026-01-01

### Added
- Initial release
- Privacy-focused web browsing
- Built-in ad blocking
- OSINT search capabilities
- Bookmark and history management
- Dark mode interface
- Minimal fingerprinting
- GTK-based native Linux application

---

## Upcoming Features
- Voice control with wake word detection
- Animated AI avatar
- Dark web monitoring alerts
- Mobile companion app
- Team collaboration features
