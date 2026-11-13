# Lavalink v4 Plugin Architecture & Packaging Reference

This reference covers the technical specifications and build requirements for Lavalink v4 plugins.

---

## 1. Plugin Configuration & Spring Boot Lifecycle

- **Spring Boot Version**: 3.3.x+ (Requires Java 17 or Java 21 LTS).
- **Service Registration**:
  Annotate custom components with `@Service` or `@Component`:
  ```java
  @Service("myPlugin")
  public class MyPlugin implements AudioPlayerManagerConfiguration {
      @Override
      public AudioPlayerManager configure(AudioPlayerManager manager) {
          manager.registerSourceManager(new MyAudioSourceManager());
          return manager;
      }
  }
  ```
- **Configuration Injection**:
  ```java
  @ConfigurationProperties(prefix = "plugins.myplugin")
  @Component
  public class MyPluginConfig {
      private boolean enabled = true;
      // getters and setters
  }
  ```

---

## 2. Manifest Properties (`lavalink-plugins`)

Every plugin JAR must contain a properties file located at `lavalink-plugins/<PluginName>-plugin.properties`:
```properties
name=my-plugin
version=1.0.0
lavalink-version=4.0.0
main-class=com.example.myplugin.MyPlugin
```
Without this file, Lavalink's `PluginManager` fails during startup with `BeanCreationException`.

---

## 3. Audio Pipeline Execution

Implement custom tracks extending `DelegatedAudioTrack` or `AudioTrack`:
```java
@Override
public void process(LocalAudioTrackExecutor executor) throws Exception {
    try (InputStream in = openStream()) {
        NonSeekableInputStream nis = new NonSeekableInputStream(in);
        if (isWebm) {
            processDelegate(new MatroskaAudioTrack(trackInfo, nis), executor);
        } else {
            processDelegate(new MpegAudioTrack(trackInfo, nis), executor);
        }
    }
}
```

---

## 4. Gradle & JitPack Pre-flight Checklist

1. **JitPack Requirements**:
   - `jitpack.yml` specifying Java target (`jdk: openjdk17` or `openjdk21`).
   - Gradle wrapper executable flag set: `git update-index --chmod=+x gradlew`.
   - `gradle-wrapper.jar` committed to repository or downloaded via `before_install` curl.
2. **Local Smoke Test**:
   - Run `./gradlew compileJava`
   - Run `./gradlew :plugin:jar` or `./gradlew assemble`
   - If available, execute with `:runLavalink`.
