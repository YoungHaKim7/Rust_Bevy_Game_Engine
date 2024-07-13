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

```
cargo add avian3d
```
- https://crates.io/crates/avian3d

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

```
cargo add avian2d
```
- https://crates.io/crates/avian2d

<hr>


# 케릭터 땅과 마진 맞추는 방법
- https://docs.rs/avian2d/latest/avian2d/dynamics/ccd/index.html

- 한국분이 질문한거 해결 이슈
  - https://github.com/Jondolf/avian/issues/412

# What Is CCD?

- https://docs.rs/avian2d/latest/avian2d/dynamics/ccd/index.html

<p>Physics simulation is typically done in a discrete manner.
At the beginning of each physics frame, the simulation checks for collisions,
and if none are found for a given rigid body, it is free to move according to
its velocity and the size of the timestep.</p>
<p>This generally works well for large or slowly moving objects, but fast and small
objects can pass through thin geometry such as walls and triangle meshes.
This phenomenon is often called <strong>tunneling</strong>.</p>
<svg width="300" height="350" viewBox="0 0 300 350" fill="none" xmlns="http://www.w3.org/2000/svg">
    <rect x="141" y="1" width="18" height="298" fill="#64C850" stroke="black" stroke-width="2"/>
    <circle cx="275" cy="150" r="24" stroke="#5064C8" stroke-width="2" stroke-dasharray="4 4"/>
    <circle cx="25" cy="150" r="24" fill="#5064C8" stroke="black" stroke-width="2"/>
    <path d="M275.707 150.707C276.098 150.317 276.098 149.683 275.707 149.293L269.343 142.929C268.953 142.538 268.319 142.538 267.929 142.929C267.538 143.319 267.538 143.953 267.929 144.343L273.586 150L267.929 155.657C267.538 156.047 267.538 156.681 267.929 157.071C268.319 157.462 268.953 157.462 269.343 157.071L275.707 150.707ZM25 151L275 151L275 149L25 149L25 151Z" fill="#E19664"/>
    <text x="150" y="325" style="fill: #b4b4b4; font: 18px monospace; text-anchor: middle;">Discrete</text>
</svg>
