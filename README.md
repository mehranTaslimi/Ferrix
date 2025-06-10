## Core Download Features

- 🟢 Multi-threaded Downloads
- 🔵 Pause / Resume Support
- 🔵 Download Scheduling
- 🔵 Batch Downloads
- 🔵 Clipboard Monitoring
- 🔵 Download Acceleration
- 🔵 Resume Broken Downloads
- 🔵 Smart File Name Handling

## Network Features

- 🔵 Proxy Support (HTTP, HTTPS, SOCKS4/5)
- 🔵 Bandwidth Throttling
- 🔵 VPN / Tor Support
- 🔵 User-Agent Spoofing

## File Handling Features

- 🔵 File Integrity Checking (MD5, SHA-1, CRC32)
- 🔵 Post-Download Actions (e.g. antivirus scan, notifications)
- 🔵 Auto Sorting by File Type
- 🔵 Archive Support (Auto-extract .zip, .rar, .7z)

## Integration Features

- 🔵 Browser Integration (Chrome, Firefox, etc.)
- 🔵 API Access / Webhooks
- 🔵 Torrent & Magnet Link Support
- 🔵 YouTube / Media Downloading Support

## Smart UX Features

- 🔵 Download Priority Control
- 🔵 Dark Mode / Custom Themes
- 🔵 Drag & Drop Support
- 🔵 Notifications (on completion or failure)
- 🔵 Error Handling with Automatic Retry

## Security & Privacy

- 🔵 HTTPS Support
- 🔵 Credential Storage / Login Support
- 🔵 Encrypted Downloads Handling
- 🔵 Private Mode (no history)

## Developer & Power User Tools

- 🔵 Command Line Interface (CLI)
- 🔵 Configurable Rules / Automation
- 🔵 Portable Mode (no install needed)
- 🔵 Cross-Platform Support (Windows, macOS, Linux)
- 🔵 Custom Script Hooks (before/after download)

## Cloud & Sync Capabilities

- 🔵 Cloud Sync (settings and queues)
- 🔵 Remote Management (web or mobile access)
- 🔵 Auto Backup (history and settings)

## Todos

1. Create better error handling using the `thiserror` crate

2. Fix streamed file behavior so the filesystem shows it's being downloaded (e.g. circular progress on macOS)

3. Parse URL and file extension to save metadata and file info

4. Implement pause and resume functionality for downloads

5. Fix progress updates and internet speed reporting for small files (currently not sent)

6. Show download start time and end time
