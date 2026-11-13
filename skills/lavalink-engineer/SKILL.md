---
name: lavalink-development
description: >-
  Universal elite skill for building, debugging, auditing, reverse-engineering, and operating Lavalink v4 plugins, Lavaplayer AudioSourceManagers, media scrapers (YouTube InnerTube, Spotify, JioSaavn, SoundCloud, Deezer), anti-bot systems (burner cookies, SAPISIDHASH, poToken, cipher solvers), source mirroring, REST/WebSocket extensions, pre-push checklists, JitPack/Maven release testing, and node ops. Forces latest live-info checks from the web, real audio stream verification, and honest Verified/Unverified reporting.
---

# Lavalink Development (Universal Elite)

Priority: **Real, tested behavior over plausible code.** Never claim something works merely because it compiles, returned HTTP 200, or "looks right".

Loop: **Inspect → Fetch latest facts from live web → Plan small → Implement → Test reality → Pre-push audit → Report honestly.**

---

## 1. Inspect First
- Read build files (`build.gradle(.kts)`, `settings.gradle(.kts)`), pinned versions (Lavalink, plugin-api, Lavaplayer, JDK/Kotlin), plugin manifest, `application.yml`, tests, CI.
- Trace the real path: `identifier → source manager → track → stream → audio frames`.
- Edit only what the task requires; follow existing package structure and design patterns.
- Preserve working behavior. Do not rewrite working code for aesthetic preference. Do not revert user changes you didn't make.

---

## 2. Always Fetch Latest Facts (Never Trust Memory)
Lavalink, Lavaplayer, youtube-source, and target media platforms change constantly. For anything version/API/behavior dependent, check live sources:
1. **Official docs**: `lavalink.dev` (REST, WebSocket, plugins, config).
2. **Upstream repos**: `lavalink-devs/Lavalink`, `lavalink-devs/lavaplayer`, `lavalink-devs/youtube-source`, `topi314/LavaSrc`.
3. **Releases, changelogs, and open issues** (anti-bot and scraping breakages show up in GitHub issues first).
4. **The version actually pinned in the project** (inspect its source/JAR, not just documentation).

Useful verification commands:
```bash
curl -s https://api.github.com/repos/lavalink-devs/Lavalink/releases/latest | grep -E '"tag_name"|"published_at"'
curl -s https://api.github.com/repos/lavalink-devs/youtube-source/releases/latest | grep -E '"tag_name"|"published_at"'
curl -s https://maven.lavalink.dev/releases/<group/path>/maven-metadata.xml
```
Never invent classes, methods, annotations, config keys, endpoints, or features. If an API cannot be verified, state it explicitly. If no network access is available, mark version-specific claims as **Unverified**.

---

## 3. Plugin Architecture & Spring Boot Lifecycle (lavalink.dev)
- Use the **public plugin API** (`dev.arbjerg.lavalink:plugin-api`); internals are discouraged.
- Manifest location: `src/main/resources/lavalink-plugins/<name>.properties` or `lavalink-plugins/<name>-plugin.properties`:
  ```properties
  name=<plugin-name>
  version=<semver>
  lavalink-version=<min-version>
  main-class=<fully-qualified-class>
  ```
- Plugins run on Spring Boot: config via `@ConfigurationProperties(prefix = "plugins.<name>")`, custom REST endpoints via `@RestController` under `/v4/<plugin-name>/...`.
- A new audio source = expose your `AudioSourceManager` as a **Spring bean** or register via `AudioPlayerManagerConfiguration`.
- Test against a real node using Gradle `:runLavalink` or a dedicated local Lavalink test runner.
- When using a custom YouTube plugin/source: **disable Lavalink's built-in YouTube source** (`lavalink.server.sources.youtube: false`) to avoid classpath and source resolution conflicts.

---

## 4. Source Managers & Audio Pipeline
- `loadItem` must claim **only** identifiers it recognizes; return `null` otherwise. Real failures throw `FriendlyException` with proper severity (`COMMON`, `SUSPICIOUS`, `FAULT`). "No match" is never an exception.
- **Cheap metadata at load time, playable URL at play time**: URLs expire. Loading a 100-track playlist must not trigger 100 media stream extractions upfront.
- **Audio Piping in Custom Tracks**:
  - Implement `process(LocalAudioTrackExecutor executor)`.
  - Pipe network streams through `NonSeekableInputStream`.
  - Route WebM/Opus data through `MatroskaAudioTrack`.
  - Route M4A/AAC data through `MpegAudioTrack`.
- Thread safety: no unsynchronized shared state; bounded caches; clean shutdown of thread pools; no blocking calls on Lavalink's request threads without explicit timeouts.
- **Mirror Sources** (Spotify/Apple/Pandora → playable audio):
  - Match by ISRC first, then normalized title + artist.
  - Validate duration tolerance ($\pm 2\text{ to } 5\text{ seconds}$).
  - Penalize live, cover, karaoke, 8D, slowed/reverb unless requested in search.
  - Maintain explicit provider fallback chains and log the winning provider.
- **Single-Flight Coalescing**: Deduplicate concurrent identical lookups to prevent self-inflicted 429 rate limits.

---

## 5. Media Scraping & Anti-Bot Reality
Treat these as completely independent variables: **IP reputation, Account, Cookies/Session, Tokens (poToken/visitorData), Headers, Client Identity, Proxy, Rate limits.** Never assume one solves the other.

### YouTube InnerTube Engineering
- **Burner Cookies & `SAPISIDHASH` (Datacenter IP Bypass)**:
  - Parse DevTools tab dumps (`ytburner.txt`) or `a=b; c=d`. If keys duplicate, keep the first.
  - Compute SHA-1 on current epoch seconds:
    $$\text{SAPISIDHASH } \langle ts \rangle\_\text{SHA1}(\langle ts \rangle + \text{" "} + \text{sapisid} + \text{" https://music.youtube.com"})$$
    Extract `sapisid` from `__Secure-3PAPISID` or `SAPISID`.
  - Client: `WEB_REMIX`, Version: `1.20260213.01.00`, Headers:
    `Authorization: SAPISIDHASH <ts>_<hex>`, `Cookie: <sanitized>`, `Origin: https://music.youtube.com`, `Referer: https://music.youtube.com/`, `X-Origin: https://music.youtube.com`, `X-YouTube-Client-Name: 67`, `X-Youtube-Bootstrap-Logged-In: true`.
- **Signature Timestamp (STS) & Cipher Decryption**:
  - Extract active player ID from `https://www.youtube.com/iframe_api` and `sts` from `base.js`.
  - Pass `sts` in `playbackContext.contentPlaybackContext.signatureTimestamp`.
  - Resolve `signatureCipher` and `n=` throttling via remote cipher microservice (`https://cipher.kikkia.dev`, `yt-cipher`) or local JS decipher bridge (`youtubei.js`).
- **Format Prioritization**:
  - Filter `adaptiveFormats` for `audio/*`. Prioritize Opus/WebM (itags 251, 250, 249) over AAC/M4A (itag 140).
- **Stream URL Expiry Caching**:
  - Parse `expire=<unix_timestamp>` from stream URL.
  - Cache stream URLs with TTL = `expire - 300s` to prevent mid-track timeouts.
- **Client Rotation (Unauthenticated)**:
  - Rotate across `ANDROID_VR`, `ANDROID_MUSIC`, `ANDROID`, `IOS`, `TVHTML5`, `WEB_EMBEDDED`.
  - Circuit break on 429/403 with exponential cooldowns (minimum 60s).
- **Third-Party Helpers are Trust Boundaries**:
  - External PoToken generators, cipher servers, or proxies can log requests, go down, or inject payloads. Make URLs configurable, set strict timeouts, never send credentials to them, and prefer self-hosting.

---

## 6. Security & Credential Hygiene
- **Zero Credentials in Git**: Enforce `.gitignore` on `*cookie*.txt`, `ytburner*.txt`, `credentials.json`, `.env`.
- **Never Output Raw Tokens**: Redact `SAPISID`, session tokens, passwords, and private cookies from logs, error messages, and terminal output.
- **SSRF Defense**: Scheme (`http`/`https`) and host allowlists. Validate resolved IP addresses; block loopback (`127.0.0.1`), link-local (`169.254.169.254`), and private subnets (`10.0.0.0/8`, `192.168.0.0/16`). Cap redirects.
- **Disk & Memory Bounds**: Bound all LRU memory caches. Use atomic writes (`.part` file rename) for disk caches with strict quota enforcement.
- **Supply Chain**: Build from source. Never trust random precompiled JARs committed to repositories.

---

## 7. Real Testing & Proof of Playback Standard
Do not accept as proof: *"it compiles"*, *"HTTP 200"*, *"the README says so"*, *"AI generated it"*.

1. **Build & Unit Tests**: Run `./gradlew compileJava` and unit tests.
2. **Node Integration**: Test with `:runLavalink` or a local node; verify discovery in `/v4/info` and startup logs.
3. **Live API Calls**:
   ```bash
   curl -s -H "Authorization: $LL_PASS" "http://localhost:2333/v4/loadtracks?identifier=<id-or-search>"
   ```
   Cover: search, track ID, playlist, invalid input, rate limit simulation.
4. **Actual Audio Delivery Verification**:
   ```bash
   curl -I -H "User-Agent: Mozilla/5.0" -H "Range: bytes=0-1024" "<stream_url>"
   ```
   Confirm `HTTP 206 Partial Content` and verify **magic bytes** (`\x1A\x45\xDF\xA3` for WebM, `ftyp` for M4A).
5. **Inspect Failures**: Check logs for swallowed exceptions, thread leaks, and socket leaks.

---

## 8. Pre-Push & Release Audit Checklist

Never push to remote without completing this exact pre-flight checklist:

### A. Pre-Push Verification (Local Machine)
1. **Never Push Without Explicit User Approval**: If the user asks to inspect, prepare, or test, do not execute `git push`. Wait for direct go-ahead.
2. **Clean Working Tree Audit**:
   - Run `git status` and `git diff`.
   - Ensure no leftover scratch scripts, test dumps, or temporary files are untracked or staged.
   - Verify `.gitignore` is intact and active for all cookie and credential patterns.
3. **Full Compilation & Assembly**:
   ```bash
   ./gradlew clean compileJava assemble
   ```
   Ensure zero compilation errors, zero broken imports, and that `plugin/build/libs/` produces the expected plugin JAR.
4. **Sync Versioning & Documentation**:
   - **Properties Manifest**: Check `lavalink-plugins/<name>-plugin.properties`. Does `version=X.Y.Z` match the new release tag?
   - **`README.md` Update**:
     - Update version badges (`vX.Y.Z`).
     - Update sample `application.yml` blocks with new configuration properties.
     - Document new configuration keys with clear descriptions and safe defaults.
   - **Changelog / Release Notes**: Prepare a clear bulleted summary of features, fixes, and behavioral changes.

### B. JitPack & Maven Post-Push Verification
When a tag or release is pushed to GitHub:
1. **JitPack Wrapper Permissions**:
   - Ensure the Gradle wrapper has Unix executable bits set:
     ```bash
     git update-index --chmod=+x gradlew
     ```
   - Ensure `gradle/wrapper/gradle-wrapper.jar` exists in the repo or `jitpack.yml` cleanly pulls it via `curl`.
2. **Verify Remote JitPack Build**:
   - Monitor the build log on JitPack:
     ```bash
     curl -s "https://jitpack.io/api/builds/com.github.<user>/<repo>/<tag>"
     ```
   - Check the raw build log:
     ```bash
     curl -s "https://jitpack.io/com/github/<user>/<repo>/<tag>/build.log"
     ```
   - Confirm status is `"status": "ok"` and `BUILD SUCCESSFUL`.
3. **Verify Downloadable Artifact in Lavalink**:
   - Test plugin download declaration in a test `application.yml`:
     ```yaml
     lavalink:
       plugins:
         - dependency: "com.github.<user>:<repo>:<tag>"
           repository: "https://jitpack.io"
     ```
   - Verify Lavalink downloads, loads, and initializes the plugin JAR without `PluginManager` class-loading failures.

---

## 9. Project Context: SolaceAudio (Nex-Devz)
*Consult when working inside the SolaceAudio repository:*
- **Architecture**: Lavalink v4 plugin, JDK 17/21+, multi-module Gradle (`main/` core engine, `plugin/` Spring Boot Lavalink adapter). Published via JitPack (`com.github.Nex-Devz:SolaceAudio:<tag>`).
- **Core Engine**: Pure Java `HttpClient`, zero bloated dependencies.
- **Key Subsystems to Preserve**:
  - `SingleFlightResolver`: Coalesces duplicate in-flight requests.
  - `TrackNormalizer` & `CandidateScorer`: Strict metadata scoring and duration tolerance.
  - `AtomicSafeCacheManager`: Atomic `.part` stream caching with quota enforcement.
  - Multi-provider fallback chain: `Deezer ISRC → YouTube ISRC → YouTube Search → SoundCloud → Bandcamp`.
  - Client rotation with health tracking and 60s cooldowns on 403/429.
  - `YtMusicCookieAuth`: Burner cookie parsing, live STS cache, and real-time `SAPISIDHASH` InnerTube `WEB_REMIX` injection.
- **Build & Verification Command**:
  ```bash
  ./gradlew compileJava && ./gradlew assemble
  ```
  Artifacts generated at `plugin/build/libs/`.

---

## 10. Reporting Format & Claim Labels
In every technical response, report with honest classification:
```text
Implemented:
- <Files and components modified>

Verified (How):
- <Actual compilation, live curl, test output, status codes>

Tests Run + Results:
- <Build output, stream verification, magic byte checks>

Known Limitations / Provider Caveats:
- <Flaky behaviors, rate limits, datacenter IP restrictions>

Unverified Assumptions:
- <Items not yet tested on real endpoints>
```
Label technical claims strictly:
- **[Verified]**: Proven by upstream source, official documentation, or live reproducible test.
- **[Likely]**: Strong technical evidence, but not directly tested in this run.
- **[Unknown]**: Insufficient evidence.
- **[Outdated]**: Previously valid, broken by recent upstream changes.
- **[False]**: Directly contradicted by reality.

**Golden Rule: Do not build what sounds right. Build what can be proven to work.**
