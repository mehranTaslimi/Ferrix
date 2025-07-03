# HTTP Request Types & Scenarios in a Download Manager (Ferrix)

This document lists all the essential HTTP request types, headers, and advanced scenarios that a modern download manager like **Ferrix** should support.

---

## ✅ Core Download Features

- [x] HTTP `GET` requests
- [ ] Follow redirects (`3xx`)
- [x] Resume downloads with `Range` headers
- [x] Parallel chunked downloads (multi-threaded with `Range`)
- [ ] Retry on failure (with backoff strategy)
- [x] Speed throttling / rate limiting

---

## 🔐 Authentication & Access

- [ ] Basic authentication
- [ ] Bearer token authentication
- [ ] Custom `Authorization` header
- [ ] Cookie-based session (e.g., login-protected links)
- [ ] API key in header or query string
- [ ] Client certificate authentication (TLS)
- [ ] Signed URL (e.g., S3 pre-signed URLs)

---

## 🌐 HTTP Header Support

- [ ] Custom headers (e.g., `User-Agent`, `Referer`, `Origin`)
- [ ] Compression: handle `gzip`, `deflate`
- [ ] `ETag` / `If-Modified-Since` support for conditional downloads
- [x] `HEAD` request support (for pre-check)
- [x] MIME type detection from `Content-Type`
- [x] Extract filename from `Content-Disposition`

---

## 🛠️ Proxy & Network

- [ ] HTTP Proxy support
- [ ] SOCKS5 Proxy support
- [ ] System proxy detection via env/OS
- [ ] Proxy with authentication

---

## 🎮 Redirection & Stream Handling

- [ ] HTTP 3xx redirect handling (follow chains)
- [ ] Chunked transfer encoding support
- [ ] Streaming content support (progressive download)
- [ ] Transfer-Encoding: chunked
- [ ] Prefetch with `HEAD`

---

## 🎥 Media & Advanced Sources

- [ ] HLS (`.m3u8`) video stream downloads
- [ ] DASH (`.mpd`) adaptive stream downloads
- [ ] Form-based downloads (via `POST`)
- [ ] JavaScript-generated download links (requires headless browser/crawler)

---

## ⚠️ Special/Error Handling

- [ ] 429 Too Many Requests — Retry with delay/backoff
- [ ] 401 Unauthorized — Handle token refresh or fail
- [ ] 403 Forbidden — Detect and log
- [ ] 5xx Errors — Retry with limits
- [ ] DNS errors and timeouts — Retry/fallback logic

---

## ✅ Request Logic Variants

- [x] Multi-threaded range requests (parallel chunks)
- [ ] Single-threaded fallback
- [ ] Conditional downloads via `ETag` / `Last-Modified`
- [ ] Retry queue with exponential backoff
- [ ] Mirror fallback (try next URL on failure)
- [ ] Auth refresh flow for expired sessions

---
