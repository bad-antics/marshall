# Architecture

## Overview

Marshall follows a multi-process architecture inspired by Chromium but implemented in Rust for memory safety.

```
┌─────────────────────────────────┐
│          Main Process           │
│  ┌──────┐ ┌────────┐ ┌──────┐  │
│  │ UI   │ │ Router │ │Config│  │
│  │Engine│ │        │ │      │  │
│  └──┬───┘ └───┬────┘ └──────┘  │
│     │         │                 │
├─────┼─────────┼─────────────────┤
│     │    IPC  │                 │
├─────┼─────────┼─────────────────┤
│  ┌──┴───┐ ┌───┴────┐ ┌──────┐  │
│  │Tab 1 │ │ Tab 2  │ │Tab N │  │
│  │WebKit│ │ WebKit │ │WebKit│  │
│  └──────┘ └────────┘ └──────┘  │
│        Renderer Processes       │
└─────────────────────────────────┘
```

## Core Components

### UI Engine (GTK3)
- Tab bar with drag-and-drop
- URL bar with autocomplete
- Sidebar panels (bookmarks, history, AI)
- Settings dialog

### Content Blocker
- EasyList/EasyPrivacy filter parsing
- WebKit content rule compilation
- Real-time request interception
- ~70,000 rules loaded at startup

### Tor Integration
- SOCKS5 proxy to local Tor daemon
- Circuit display in toolbar
- New identity per-tab option
- `.onion` address support

### Tab Isolation
- Separate WebKit process per tab
- No shared cookies between tabs
- Independent JavaScript contexts
- Memory limit per renderer

## Data Flow

```
User Input → URL Bar → Router → Content Blocker → WebKit Renderer → Display
                                      ↓
                               Block/Allow Decision
                                      ↓
                              Privacy Dashboard Update
```
