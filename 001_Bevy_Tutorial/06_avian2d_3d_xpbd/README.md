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

![Screenshot from 2024-07-13 15-40-07](https://github.com/user-attachments/assets/9b4d0e16-a068-47be-85db-7e6ea9acf0cf)
Discrete
![Screenshot from 2024-07-13 15-40-14](https://github.com/user-attachments/assets/09a091b1-c45f-4b01-a973-51bf0b4e8863)
![Screenshot from 2024-07-13 15-40-24](https://github.com/user-attachments/assets/fdef78dc-09ad-43c2-a231-7f8167484e1e)
![Screenshot from 2024-07-13 15-40-32](https://github.com/user-attachments/assets/87bb7bed-1b8d-4e2c-97da-365c1bb2c945)

