# Rust_Bevy_getting-started

- https://bevyengine.org/learn/quick-start/getting-started/ecs/

- https://github.com/bevyengine/bevy/tree/latest/examples#examples

# 시리즈 모아보기

- Bevy Materials Logic Projects
  - https://youtube.com/playlist?list=PLT_D88-MTFOMNRPAC-62Hz096aIjT4Noy&si=iuS_sLwD1ISOFg3j

- This Week in Bevy Engine | chris biscardi
  - https://youtube.com/playlist?list=PLWtPciJ1UMuAyAER9ASVEDRIz0DUspOeZ&si=yOnNpHaOaEhQkbtZ

# Bevy Rendering Demystified | Logic Projects

- https://youtu.be/5oKEPZ6LbNE?si=i3CQaR1_RWy2RV6C



# Bevy 0-14-rc.2, Powerglove, and Soup - This Week in Bevy chris biscardi
- https://youtu.be/5r90Z7Ec3Pw?si=8MLWedGjtK-x1BFS


# 컴파일 빠르게 만들기
- Faster linking times on nightly on Linux using `rust-lld`
  - https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html

- `.cargo/config.toml`
```toml

[build]
rustflags = ["-Z", "threads=20"]

[target.x86_64-unknown-linux-gnu]
rustflags = ["-Zlinker-features=-lld"]
```

- `rust-toolchain.toml` 

```toml

[toolchain]
channel ="nightly"
components = ["rustfmt", "rust-src"]
```
