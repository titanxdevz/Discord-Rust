# Media Scraping & InnerTube Reference Protocols

This reference outlines production protocols for extracting audio streams from YouTube Music and other audio platforms without relying on hallucinated workarounds.

---

## 1. YouTube InnerTube & Burner Account Authentication

### Datacenter IP Bypass with Burner Cookies
When Lavalink nodes run on cloud/datacenter IPs (Hetzner, OVH, DigitalOcean), YouTube blocks unauthenticated requests and PoToken challenges often fail.

**Working Authenticated Protocol**:
1. **Cookie Ingestion**:
   - Parse cookie dumps (Chrome DevTools tab-separated or standard `a=b; c=d`).
   - If keys are duplicated, retain the first occurrence.
2. **`SAPISIDHASH` Generation**:
   - Locate `__Secure-3PAPISID` or `SAPISID`.
   - Calculate SHA-1 on current epoch seconds:
     $$\text{SAPISIDHASH } \langle ts \rangle\_\text{SHA1}(\langle ts \rangle + \text{" "} + \text{sapisid} + \text{" https://music.youtube.com"})$$
3. **InnerTube Player Call (`WEB_REMIX`)**:
   - Endpoint: `POST https://music.youtube.com/youtubei/v1/player?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8`
   - Client: `WEB_REMIX`, Version: `1.20260213.01.00`
   - Headers:
     ```http
     Authorization: SAPISIDHASH <ts>_<hex>
     Cookie: <sanitized_cookie_header>
     Origin: https://music.youtube.com
     Referer: https://music.youtube.com/
     X-Origin: https://music.youtube.com
     X-YouTube-Client-Name: 67
     X-YouTube-Client-Version: 1.20260213.01.00
     X-Youtube-Bootstrap-Logged-In: true
     User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36
     ```
4. **Signature Timestamp (STS)**:
   - Obtain from live `https://www.youtube.com/iframe_api` -> `base.js` -> `signatureTimestamp:`.
   - Pass in `playbackContext.contentPlaybackContext.signatureTimestamp`.

---

## 2. Client Fallback Hierarchy (When Unauthenticated)

When operating without session cookies, rotate clients with circuit breakers on 429/403:
1. `ANDROID_VR` (Key: Android Key, minimal bot challenge rate)
2. `ANDROID_MUSIC`
3. `ANDROID` (Header: `X-Goog-Api-Format-Version: 2`)
4. `IOS`
5. `TVHTML5` (`thirdParty: https://www.youtube.com`)
6. `WEB_EMBEDDED` (`Referer: https://www.youtube.com`)

---

## 3. Format Filtering & Cipher Decryption

1. **Format Selection**:
   - Filter `adaptiveFormats` where `mimeType` starts with `audio/`.
   - Sort Opus/WebM (`audio/webm; codecs="opus"`, itags 251, 250, 249) first, then by highest `bitrate`.
2. **Cipher Deciphering & `n`-Parameter Solving**:
   - If format has `signatureCipher` / `cipher` or URL contains throttling parameter `n=`:
     - Delegate to a remote cipher instance (`https://cipher.kikkia.dev` or `yt-cipher`) or local JS runtime with `youtubei.js` decipher function.

---

## 4. Audio Stream Verification Test

To verify playable direct audio streams via cURL:
```bash
curl -I -H "User-Agent: Mozilla/5.0" -H "Range: bytes=0-1024" "<stream_url>"
```
Expected: `HTTP 206 Partial Content`, `Content-Range: bytes 0-1024/...`, `Content-Type: audio/webm` or `audio/mp4`.
