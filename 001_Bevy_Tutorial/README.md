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
