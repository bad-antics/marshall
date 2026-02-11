# Privacy Features

## Ad & Tracker Blocking

Marshall blocks ads and trackers using compiled content rules:
- **EasyList** — Ad blocking rules
- **EasyPrivacy** — Tracker blocking
- **Fanboy's Annoyance** — Cookie notices, social widgets
- **Custom rules** — Add your own in Settings > Content Blocking

### Stats
Blocks tracked in real-time and displayed in the privacy dashboard.

## Anti-Fingerprinting

| Technique | Protection |
|-----------|-----------|
| Canvas | Returns randomized pixel data |
| WebGL | Spoofs renderer/vendor strings |
| AudioContext | Adds noise to audio fingerprint |
| Fonts | Reports limited standard font set |
| Screen | Rounds dimensions to common values |
| User-Agent | Rotates among common browsers |
| Timezone | Can be overridden per-tab |

## Cookie Management

- **Strict mode**: Block all third-party cookies
- **Balanced mode**: Block known tracking cookies
- **Permissive mode**: Standard browser behavior
- **Auto-clear**: Purge cookies on tab close

## HTTPS Enforcement

- Automatic HTTP → HTTPS upgrade
- HSTS preload list included
- Certificate transparency checking
- Mixed content blocking

## DNS over HTTPS

Built-in DoH support with configurable providers:
- Cloudflare (1.1.1.1)
- Google (8.8.8.8)
- Quad9 (9.9.9.9)
- Custom DoH endpoint
