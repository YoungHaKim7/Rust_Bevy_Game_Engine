# avian2d & 3d NVIDIA저작권때문에 살짝 코드 바꿔서 나옴.(xpbd)

- https://docs.rs/avian2d/latest/avian2d/
  - https://github.com/Jondolf/avian

- 2D Examples
  - https://github.com/Jondolf/avian/tree/main/crates/avian2d/examples

- 3D Examples
  - https://github.com/Jondolf/avian/tree/main/crates/avian3d/examples

# Introducing Avian Physics 0.1
The next evolution of ECS-driven physics for Bevy
- July 6, 2024
  - https://joonaa.dev/blog/06/avian-0-1

<hr>

# 기본틀(3d 물리엔진)

- Cargo.toml
```toml
[dependencies]
bevy = "0.14.0"
avian3d = { git = "https://github.com/Jondolf/avian", branch = "main" }

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

# 기본틀(2d물리엔진)

- Cargo.toml
```toml
# For 2D applications:
[dependencies]
avian2d = "0.1"

# For 3D applications:
[dependencies]
avian3d = "0.1"

# If you want to use the most up-to-date version, you can follow the main branch:
[dependencies]
avian3d = { git = "https://github.com/Jondolf/avian", branch = "main" }

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```
- https://github.com/Jondolf/avian

<hr>


# 케릭터 땅과 마진 맞추는 방법
- https://docs.rs/avian2d/latest/avian2d/dynamics/ccd/index.html

- 한국분이 질문한거 해결 이슈
  - https://github.com/Jondolf/avian/issues/412
