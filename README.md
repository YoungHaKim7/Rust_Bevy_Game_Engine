# Rust_Bevy_Game_Engine

# Rust Game Dev.(Rust, Rust_Bevy, Blender)
<p align="center">
  <img width=120px src="https://user-images.githubusercontent.com/67513038/213436632-820a1675-98d9-4626-979d-be63c60cdcb7.png" hspace="20"/>
  <img width=200px src="https://bevyengine.org/assets/bevy_logo_dark.svg" hspace="20" />
  <br><img width=120px src="https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/b96e6f3b-f5ba-4b3d-8f13-501b8d7b9870" hspace="20"/>
</p>

<hr>

# link

- [(통합본)rust_game_dev--globalyoung정리중-모아보기](#rust_game_dev--globalyoung정리중-모아보기)


<hr />

- [(250530)(유튜브 설명 외부영상) Start a new Bevy 2d Game with the Bevy CLI | chris biscardi](https://youtu.be/ez4oGeM3X2o?si=mTP1GH1KtvTj17OU)
  - This template is a great way to get started on a new 2D Bevy game!
    -  https://github.com/TheBevyFlock/bevy_new_2d


<hr />

- [Migration-guides_https://bevyengine.org/learn/migration-guides/introduction/](https://bevyengine.org/learn/migration-guides/introduction/)
  - 0.14 -> 0.15
    - [https://bevyengine.org/learn/migration-guides/0-14-to-0-15/](https://bevyengine.org/learn/migration-guides/0-14-to-0-15/)
  - 0.13 -> 0.14
    - [https://bevyengine.org/learn/migration-guides/0-13-to-0-14/](https://bevyengine.org/learn/migration-guides/0-13-to-0-14/)



<hr>

- [러스트_베비_게임엔진_Bevy-gameengine-최신-소식This-Week-in-Bevy](#bevy-gameengine-최신-소식thisweekinbevy)
- [Bevy버젼업_되면_여기부터_체크_rust-bevy-game-engine-migration-guide](#rust-bevy-game-engine-migration-guide)

<hr>

- Assets파일 구성하기 기본 뼈대
  - [rust-게임개발-기본-구성assets폴더-구성하기](#rust-게임개발-기본-구성assets폴더-구성하기)

- [color컬러-코딩이해하기RGB이해](#color컬러-코딩이해하기)
  - [bevy-0.14-color-코딩-패턴snippets](#bevy-014-color-코딩-패턴snippets)
    - [실제-현실에-가까운-assets만들기-잔디를-고화질-카메라로-찍은-후에-assets파일-만들기_게임인지 현실인지 모르게 만들어보자](#실제-현실에-가까운-assets만들기-잔디를-고화질-카메라로-찍은-후에-assets파일-만들기)

- [rust--blender-로-게임-만드는-demo영상](#rust--blender-로-게임-만드는-demo영상)

<hr>

- [Blender_블렌더_최신소식_](https://github.com/YoungHaKim7/Rust_Game_Blender?tab=readme-ov-file#blender%EC%B5%9C%EC%8B%A0-%EC%86%8C%EC%8B%9D)

<hr>

- [C++코드지만 볼만함. qt-tutorials](#qt-tutorialsc)

<hr>

- [러스트로 오래 살아남은 case VS. 러스트하다가 포기한 케이스](#러스트해서-살아남은-case-)

<hr>

- [#국내--it-개발-전반과-게임-개발에-관한-소식과-정보를-공유하는-gpgstudycom입니다](#국내--it-개발-전반과-게임-개발에-관한-소식과-정보를-공유하는-gpgstudycom입니다)

<hr>

# Are we Game yet?[[🔝]](#link)

https://arewegameyet.rs/

- 게임 종합적으로 다 정리중(Rust)
  - https://github.com/YoungHaKim7/Rust_Game_Blender

# 게임 물리엔진[[🔝]](#link)
- https://arewegameyet.rs/ecosystem/physics/

<hr>

## Game엔진들 다른 언어라 다 있다  굿 굿[[🔝]](#link)

- https://www.dragonflydb.io/game-dev/engines/rust

<hr>

# 현대 게임 엔진들이 사용하는 표준 방식[[🔝]](#link)
- Unreal Engine : `Texture2DArray` 사용
- Unity URP : `Texture2DArray` 사용
- Frostbite : `Texture2DArray` 사용

- 특히 Deferred Rendering을 사용하는 현재 프레임워크에서는 Texture2DArray + 한번에 처리가 훨씬 효율적입니다.

<hr />


# Color컬러 코딩이해하기[[🔝]](#link)

# sRGB Convert[[🔝]](#link)
- https://www.easyrgb.com/en/convert.php#inputFORM

  - https://convertingcolors.com/cmyk-color-0.13_0.13_0.00_0.31.html
    - `CMYK(0.13, 0.13,0.00,0.31)`
  ```rs
  const COLOR_BACKGROUND: Color = Color::rgb(0.13, 0.13, 0.23);
  ```

- sRGB계산법
  - Eng. Ver.
    - https://learn.microsoft.com/en-us/windows/win32/direct3d10/d3d10-graphics-programming-guide-resources-data-conversion
  - https://learn.microsoft.com/ko-kr/windows/win32/direct3d10/d3d10-graphics-programming-guide-resources-data-conversion

- RGB 누가 만든거 대단
  - https://rockyshore73.tistory.com/2    

- 색 전문가 github https://github.com/Myndex

- Convert RGB to sRGB?
  - https://stackoverflow.com/questions/35952564/convert-rgb-to-srgb
  - 소수점으로 쓰는거
    - https://docs.opencv.org/3.1.0/de/d25/imgproc_color_conversions.html
- https://www.rapidtables.com/web/color/RGB_Color.html

![color_roadmap](https://github.com/YoungHaKim7/Cpp_Training/assets/67513038/205a8f09-f3e6-4a4d-91d2-7d3284a40882)
- https://community.adobe.com/t5/premiere-pro-discussions/premiere-pro-%EA%B2%8C%EC%8B%9C%ED%8C%90%EC%97%90%EC%84%9C-%EA%B0%80%EC%9E%A5-%EB%A7%8E%EC%9D%B4-%EB%B3%B8-%EA%B2%8C%EC%8B%9C%EB%AC%BC-top-5-%ED%95%9C%EA%B5%AD%ED%8E%B8/td-p/14573189?profile.language=ko

<hr>

# (실제 현실에 가까운 Assets만들기) 잔디를 고화질 카메라로 찍은 후에 Assets파일 만들기[[🔝]](#link)
- https://youtu.be/7Um3FaXJixg?si=CZWSId9pGWduhYVA

<hr>

- 실제 현실인지 분간이 안가는 신작 FPS 게임이 나왔다... 미친거 아니야 요즘 기술력? | 00231
  - https://youtu.be/UgAqWwDOXUw?si=_p7DzBKFHqVbzKDU

<hr>

# Bevy 0.16 바뀌거

- I'm unable to fully resolve the compilation errors without more specific
  - information about the Bevy 0.16.1 API changes.
- The persistent errors indicate that `SpriteBundle`, `Camera2dBundle`, `TextStyle`, `TextBundle`, and `TextSection` are not found in scope, even with various import attempts. This suggests their module paths or names have changed significantly in Bevy 0.16.1.

- Additionally, the `no field sections` error for `bevy::prelude::Text` remains, implying that the way to modify text content has also changed.

  - To proceed, I would need to know:
    - * The correct import paths for `SpriteBundle`, `Camera2dBundle`, `Text`, `TextBundle`, `TextSection`, and `TextStyle` in Bevy 0.16.1.
    - * The new method for updating the content of a `Text` component, as direct access to `text.sections` seems to be deprecated or removed.
- If you can provide a link to the Bevy 0.16.1 migration guide or relevant documentation, I can use the `web_fetch` tool to get the necessary information.

- - 더 구체적이지 않으면 컴파일 오류를 완전히 해결할 수 없습니다
  - Bevy 0.16.1 API 변경 사항에 대한 정보.
- 지속적인 오류는 'SpriteBundle', 'Camera2dBundle', 'TextStyle', 'TextBundle', 'TextSection'이 다양한 가져오기 시도를 했음에도 불구하고 범위를 찾을 수 없음을 나타냅니다. 이는 베비 0.16.1에서 모듈 경로나 이름이 크게 변경되었음을 시사합니다.

- 또한 'bevy::prelude::Text'에 대한 '필드 섹션 없음' 오류가 남아 있어 텍스트 내용을 수정하는 방법도 변경되었음을 시사합니다.

  - 진행하려면 알아야 합니다:
    - * 베비 0.16.1에서 'SpriteBundle', 'Camera2dBundle', 'Text', 'TextBundle', 'TextSection', 'TextStyle'의 올바른 가져오기 경로입니다.
    - * 'text.sections'에 직접 접근할 수 있는 'Text' 구성 요소의 내용을 업데이트하는 새로운 방법이 사용되지 않거나 삭제된 것으로 보입니다.
- Bevy 0.16.1 마이그레이션 가이드 또는 관련 문서에 대한 링크를 제공해 주실 수 있다면, 필요한 정보를 얻기 위해 'web_fetch' 도구를 사용할 수 있습니다.

<hr />

# Bevy 0.14 Color 코딩 패턴(snippets)[[🔝]](#link)

- 변환은 여기서 해서 색깔을 찾으면 된다. 굿
  - https://www.easyrgb.com/en/convert.php#inputFORM

- https://bevyengine.org/learn/migration-guides/introduction/
  - https://bevyengine.org/learn/migration-guides/0-13-to-0-14/

```rs
// 상수로 지정해 쓰는 스타일
const COLOR_BACKGROUND: Color = Color::srgba(0.29, 0.31, 0.41, 1.0);
const COLOR_PLATFORM: Color = Color::srgba(0.13, 0.13, 0.23, 1.0);
const COLOR_PLAYER: Color = Color::srgba(0.60, 0.55, 0.60, 1.0);

const LIME_GREEN_COLOR: Color = Color::srgba(0.19608, 0.80392, 0.01961, 1.0);
const AQUA_COLOR: Color = Color::hsl(180.0, 1.00, 0.5);
const INDIAN_RED: Color = Color::srgba(0.80392, 0.36078, 0.36078, 1.0);


// --------------
// sRBG로 컬러 지정
// 그냥 선언해서 1회용으로 쓰는 스타일
// LIME_GREEN
Color::srgba(0.19608, 0.80392, 0.01961, 1.0);

// INDIAN_RED
Color::srgba(0.80392, 0.36078, 0.36078, 1.0);


// ------
// hsl로 컬러 지정
// AQUA_COLOR
Color = Color::hsl(180.0, 1.00, 0.5);
```

- Bevy 0.13 스타일 코딩
```rs
const COLOR_FLOOR: Color = Color::rgb(0.45, 0.55, 0.66);

const LIME_GREEN_COLOR: Color = Color::LIME_GREEN;
```

# Rust Bevy Game Engine Migration Guide[[🔝]](#link)

- https://bevyengine.org/learn/migration-guides/introduction/

- Migration Guide: 0.12 to 0.13
  - https://bevyengine.org/learn/migration-guides/0-12-to-0-13/

<hr>

# Rust 게임개발 기본 구성(Assets폴더 구성하기)[[🔝]](#link) 

- 외국 사람의 Github 참고 

  - https://github.com/DigitalExtinction/Game

- Assets 파일 대략 구성 

```
$ tree
.
├── audio
│   ├── music
│   │   └── menu_loop.mp3
│   └── sounds
│       ├── construct.ogg
│       ├── destruction_building.ogg
│       ├── destruction_unit.ogg
│       ├── laser.ogg
│       └── manufacture.ogg
├── fonts
│   └── Fira_Mono
│       ├── FiraMono-Medium.ttf
│       └── LICENSE
├── maps
│   ├── 8a9d5f0e522cc1aac64c45f0d4da353eccb410a00c04c84a23788e5ca5c01e2e.dem.tar
│   └── c653d17ba9a26c2d58c8a8723f37c881971207c330853764441a16df35ec7521.dem.tar
├── models
│   ├── attacker.glb
│   ├── base.glb
│   ├── pole.glb
│   ├── powerhub.glb
│   └── tree.glb
├── objects
│   ├── attacker.obj.json
│   ├── base.obj.json
│   ├── powerhub.obj.json
│   └── tree.obj.json
├── shaders
│   ├── bar.wgsl
│   ├── rally_point.wgsl
│   ├── terrain.wgsl
│   └── trail.wgsl
└── textures
    ├── skybox.png
    └── terrain.png

11 directories, 25 files
```

- ogg 소리 파일 공부 https://cloudinary.com/guides/video-formats/ogg-format-an-in-depth-look

<hr>

# Rust_Game_Dev | GlobalYoung(정리중.. 모아보기)[[🔝]](#link)

- https://github.com/YoungHaKim7/Rust_Game_Blender
- 내가 만든 영상 모아보기(Rust Game Dev.)
  - https://youtube.com/playlist?list=PLcMveqN_07mY5cEcTgC4ICHnla6LSVtnh&si=rtaqCCGuZhKpHLJs

<hr>

# BevyEngine(Rust)[[🔝]](#link)

- Bevy Github
  - https://github.com/bevyengine/bevy

- https://bevyengine.org/

  - https://bevyengine.org/learn/
  - https://bevy-cheatbook.github.io/introduction.html
  - https://bevy-cheatbook.github.io/


- https://github.com/bevyengine/bevy/tree/latest/examples#examples

<hr>

# Bevy GameEngine 최신 소식(thisweekinbevy)[[🔝]](#link)
- https://thisweekinbevy.com/

<hr>

# Bevy GameEngine 최신 소식[[🔝]](#link)

- Bevy 0-14-rc.2, Powerglove, and Soup - This Week in Bevy chris biscardi
  - https://youtu.be/5r90Z7Ec3Pw?si=8MLWedGjtK-x1BFS
- https://bevyengine.org/news/bevy-0-13/
- SME Announcements
  - https://bevyengine.org/news/sme-announcements/
- https://bevyengine.org/news/bevy-webgpu/

- This Week in Bevy Engine | chris biscardi
  - https://youtube.com/playlist?list=PLWtPciJ1UMuAyAER9ASVEDRIz0DUspOeZ&si=T-HJLRn39NKK1Hyp

- Posted on November 04, 2023 by Bevy Contributors(23.11.04)
  - https://bevyengine.org/news/bevy-0-12/

- Bevy 0.12 Transmission Demo | Marco Buono
  - https://youtu.be/t1XdxZKZ-us?si=dAWiLk0JiIUdimI0

<br>
 
<hr>

# Rust + Blender 로 게임 만드는 Demo영상

- Making an FPS game with Bevy and Rust!
  - https://youtu.be/06M2lT_I11c?si=ACv_8jUDmrWv2iXE

- Bevy Top Down Shooter Tutorial With Massive Enemy Hordes | Bones AI
  - https://youtu.be/p8d8TKo59LU?si=pbbyj9sUOmSmWgNu

<br>

<hr>

# Rust_Bevy[[🔝]](#link)

# Should you use heron or bevy rapier?[[🔝]](#link)

- heron vs bevy rapier??

https://youtu.be/zvLWibkWcVg?si=ibJVzarMVenqYEWP

# Rapier Physics Engine Showcase: Rust Physics Engine for Bevy[[🔝]](#link)

https://youtu.be/GwlZ5EPu8l0?si=cPHJUupEsMuYDDfU

# Bevy Plugin Showcases | Logic Projects[[🔝]](#link)

- https://youtube.com/playlist?list=PLT_D88-MTFOP4QE1D0eUKlPpUi35uVvuy&si=AfjBDeqcKtvHiTR0

<hr>

# Doom 만들면서 게임엔진 이해하기(Rust Code)[[🔝]](#link)
- https://gitlab.com/flukejones/room4doom
  - https://ljones.dev/blog/room4doom-20220529/
 

# Qt Tutorials(C++)[[🔝]](#link)
- https://github.com/kelvins/qt-examples-and-tutorials
- qt공식 사이트
  - C++ 예재
    - https://doc.qt.io/qt-6/qt3d-simple-cpp-example.html

# gnutplot(linuxOS 설치)
```
sudo apt install -y gnuplot
```

<hr>

<hr>

# 러스트해서 살아남은 case 👍

# 러스트 게임 개발자(240717기준) 최근까지 살아남은 몇 안되는 Rust Dev.존경스럽다.[[🔝]](#link)
- Game dev in Rust - some notes on the mess
  - https://users.rust-lang.org/t/game-dev-in-rust-some-notes-on-the-mess/104939

## 러스트하다가 포기한 case 👎

# 러스트 동시실행에서 무너진 개발자(거의 다 왔는데 바보...)(240426기준글)[[🔝]](#link)
- **[GN⁺: Rust로 게임 개발을 한 3년 후에 떠나며](<https://news.hada.io/topic?id=14521&utm_source=discord&utm_medium=bot&utm_campaign=1480>)**
- Rust에 익숙해지면 모든 문제가 사라질 것이라는 주장에 대해  
  - Rust에 익숙해져도 근본적인 문제는 사라지지 않음  
  - 게임은 복잡한 상태 머신이고 요구사항이 계속 바뀌기 때문에 Rust의 정적이고 과도하게 검사하는 특성과 맞지 않음  
  - 코드를 계속 리팩토링해야 하는 문제는 self-inflicted임  
- ...

<hr>

# 국내 🇰🇷 IT 개발 전반과 게임 개발에 관한 소식과 정보를 공유하는 GpgStudy.com입니다.[[🔝]](#link)
- https://gpgstudy.com/
