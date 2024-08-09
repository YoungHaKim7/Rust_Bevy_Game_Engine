# `Cargo.toml`일반적인 패턴

```toml
[dependencies]
bevy = {version = "0.14.0-dev"}

[profile.dev]
opt-level = 1

[profile.dev.package."*"]
opt-level = 3
```

# 컴파일 빠르게 만들기
- Faster linking times on nightly on Linux using `rust-lld`
  - https://blog.rust-lang.org/2024/05/17/enabling-rust-lld-on-linux.html

- `.cargo/config.toml`
```toml

[build]
rustflags = ["-Z", "threads=8"]

[target.x86_64-unknown-linux-gnu]
rustflags = ["-Zlinker-features=-lld"]
```

- `rust-toolchain.toml` 

```toml

[toolchain]
channel ="nightly"
components = ["rustfmt", "rust-src"]
```

<hr>

# ecs가 요즘 핫한 이유(240809)
- 데이터 컨테이너랑 시스템을 나눠서 설계하는거임.
  - https://www.delltechnologies.com/asset/ko-kr/products/storage/industry-market/h14071-ecs-architectural-guide-wp.pdf


# sRGB Convert
  - https://www.easyrgb.com/en/convert.php#inputFORM

  - https://convertingcolors.com/cmyk-color-0.13_0.13_0.00_0.31.html
    - `CMYK(0.13, 0.13,0.00,0.31)`
  ```rs
  const COLOR_BACKGROUND: Color = Color::rgb(0.13, 0.13, 0.23);
  ```

- Convert RGB to sRGB?
  - https://stackoverflow.com/questions/35952564/convert-rgb-to-srgb
- https://www.rapidtables.com/web/color/RGB_Color.html

<hr>

<hr>

# Bevy Github

- https://github.com/bevyengine/bevy

<hr>


# Bevy API
- https://docs.rs/bevy/0.14.0-rc.4/bevy/index.html
- Migration-guides
  - https://bevyengine.org/learn/migration-guides/0-12-to-0-13/

<hr>

# Bevy + Blender
- https://github.com/rust-adventure/yt-bevy-blender

# Rust By Example
- [https://doc.rust-lang.org/rust-by-example/index.html](https://doc.rust-lang.org/rust-by-example/)

<hr>

# Rust_Bevy_getting-started

- https://affanshahid.dev/posts/learning-game-dev-bevy-1/
- https://bevy-cheatbook.github.io/

- https://bevyengine.org/learn/quick-start/getting-started/ecs/

- https://github.com/bevyengine/bevy/tree/latest/examples#examples


# Awesome-Bevy
- https://github.com/Anshorei/awesome-bevy

# 시리즈 모아보기

- Bevy Materials Logic Projects
  - https://youtube.com/playlist?list=PLT_D88-MTFOMNRPAC-62Hz096aIjT4Noy&si=iuS_sLwD1ISOFg3j

- This Week in Bevy Engine | chris biscardi
  - https://youtube.com/playlist?list=PLWtPciJ1UMuAyAER9ASVEDRIz0DUspOeZ&si=yOnNpHaOaEhQkbtZ

# Bevy Rendering Demystified | Logic Projects

- https://youtu.be/5oKEPZ6LbNE?si=i3CQaR1_RWy2RV6C



# Bevy 0-14-rc.2, Powerglove, and Soup - This Week in Bevy chris biscardi
- https://youtu.be/5r90Z7Ec3Pw?si=8MLWedGjtK-x1BFS

<hr>

![handedness](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/4a240cb3-5504-4bfa-926c-a6c511814204)

- https://affanshahid.dev/posts/learning-game-dev-bevy-1/

- https://bevy-cheatbook.github.io/

<hr>

![Screenshot 2024-06-27 at 7 36 27 PM](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/857c8a4d-68ef-4fdb-9413-8077feaf7370)

```rs
    let shapes = [
        Mesh2dHandle(meshes.add(Circle { radius: 50.0 })),
        Mesh2dHandle(meshes.add(CircularSector::new(50.0, 1.0))),
        Mesh2dHandle(meshes.add(CircularSegment::new(50.0, 1.25))),
        Mesh2dHandle(meshes.add(Ellipse::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Annulus::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Capsule2d::new(25.0, 50.0))),
        Mesh2dHandle(meshes.add(Rhombus::new(75.0, 100.0))),
        Mesh2dHandle(meshes.add(Rectangle::new(50.0, 100.0))),
        Mesh2dHandle(meshes.add(RegularPolygon::new(50.0, 6))),
        Mesh2dHandle(meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        ))),
    ];

```

<hr>

- `change_detection.rs`
  - https://github.com/bevyengine/bevy/blob/main/crates/bevy_ecs/src/change_detection.rs
```rs
//! Types that detect when their internal data mutate.

use crate::{
    component::{Tick, TickCells},
    ptr::PtrMut,
    system::Resource,
};
use bevy_ptr::{Ptr, UnsafeCellDeref};
use std::mem;
use std::ops::{Deref, DerefMut};

/// The (arbitrarily chosen) minimum number of world tick increments between `check_tick` scans.
///
/// Change ticks can only be scanned when systems aren't running. Thus, if the threshold is `N`,
/// the maximum is `2 * N - 1` (i.e. the world ticks `N - 1` times, then `N` times).
///
/// If no change is older than `u32::MAX - (2 * N - 1)` following a scan, none of their ages can
/// overflow and cause false positives.
// (518,400,000 = 1000 ticks per frame * 144 frames per second * 3600 seconds per hour)
pub const CHECK_TICK_THRESHOLD: u32 = 518_400_000;

/// The maximum change tick difference that won't overflow before the next `check_tick` scan.
///
/// Changes stop being detected once they become this old.
pub const MAX_CHANGE_AGE: u32 = u32::MAX - (2 * CHECK_TICK_THRESHOLD - 1);

/// Types that can read change detection information.
/// This change detection is controlled by [`DetectChangesMut`] types such as [`ResMut`].
///
/// ## Example
/// Using types that implement [`DetectChanges`], such as [`Res`], provide
/// a way to query if a value has been mutated in another system.
///
/// ```
/// use bevy_ecs::prelude::*;
///
/// #[derive(Resource)]
/// struct MyResource(u32);
///
/// fn my_system(mut resource: Res<MyResource>) {
///     if resource.is_changed() {
///         println!("My component was mutated!");
///     }
/// }
/// ```
```

- Types that detect when their internal data mutate.
  - 내부 데이터가 변경되는 경우를 탐지하는 유형입니다.
- The (arbitrarily chosen) minimum number of world tick increments between `check_tick` scans.

 Change ticks can only be scanned when systems aren't running. Thus, if the threshold is `N`,
 the maximum is `2 * N - 1` (i.e. the world ticks `N - 1` times, then `N` times).

 If no change is older than `u32::MAX - (2 * N - 1)` following a scan, none of their ages can
 overflow and cause false positives.
 (518,400,000 = 1000 ticks per frame * 144 frames per second * 3600 seconds per hour)
   - 'check_tick' 검색 사이의 (임의로 선택한) 최소 월드 틱 증분 수.

변경 틱은 시스템이 실행되지 않을 때만 스캔할 수 있습니다. 따라서 임계값이 'N'이면,
최대치는 2 * N - 1(즉, 세계 틱은 N - 1번, 그 다음은 N번)입니다.

스캔 후 'u32::MAX - (2 * N - 1)'보다 오래된 변경 사항이 없는 경우 해당 연령대는 변경할 수 없습니다
오버플로 및 오탐을 유발합니다.
(518,400,000 = 프레임당 1000 틱 * 초당 144 프레임 * 시간당 3600초)

- The maximum change tick difference that won't overflow before the next `check_tick` scan.

Changes stop being detected once they become this old.
pub const MAX_CHANGE_AGE: u32 = u32::MAX - (2 * CHECK_TICK_THRESHOLD - 1);

Types that can read change detection information.
 This change detection is controlled by [`DetectChangesMut`] types such as [`ResMut`].

## Example
Using types that implement [`DetectChanges`], such as [`Res`], provide a way to query if a value has been mutated in another system.

  - 다음 'check_tick' 스캔 전에 넘치지 않는 최대 변경 틱 차이입니다.

변경 사항은 이렇게 오래되면 감지되지 않습니다.
pub const MAX_CHANGE_AGE: u32 = u32::MAX - (2 * CHECK_TICK_THRESH - 1);

변경 탐지 정보를 읽을 수 있는 유형입니다.
이 변경 감지는 [[ResMut]]와 같은 [[DetectChangesMut]] 유형에 의해 제어됩니다.

## 예제
[Res]와 같이 [[DetectChanges]를 구현하는 유형을 사용하여 값이 다른 시스템에서 변경되었는지 조회하는 방법을 제공합니다.

<hr>

