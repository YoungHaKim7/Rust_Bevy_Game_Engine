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

<hr>

- [러스트_베비_게임엔진_Bevy-gameengine-최신-소식This-Week-in-Bevy](#bevy-gameengine-최신-소식thisweekinbevy)

<hr>

- [color컬러-코딩이해하기RGB이해](#color컬러-코딩이해하기)
  - [bevy-0.14-color-코딩-패턴snippets](#bevy-014-color-코딩-패턴snippets)
  - [실사에-가까운-assets만들기-잔디를-고화질-카메라로-찍은-후에-assets파일-만들기_게임인지 현실인지 모르게 만들어보자](#실사에-가까운-assets만들기-잔디를-고화질-카메라로-찍은-후에-assets파일-만들기)

- [rust--blender-로-게임-만드는-demo영상](#rust--blender-로-게임-만드는-demo영상)

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

# Color컬러 코딩이해하기[[🔝]](#link)

# sRGB Convert[[🔝]](#link)
- https://www.easyrgb.com/en/convert.php#inputFORM
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

# (실사에 가까운 Assets만들기) 잔디를 고화질 카메라로 찍은 후에 Assets파일 만들기
- https://youtu.be/7Um3FaXJixg?si=CZWSId9pGWduhYVA

<hr>

- 실제 현실인지 분간이 안가는 신작 FPS 게임이 나왔다... 미친거 아니야 요즘 기술력? | 00231
  - https://youtu.be/UgAqWwDOXUw?si=_p7DzBKFHqVbzKDU

<hr>

# Bevy 0.14 Color 코딩 패턴(snippets)[[🔝]](#link)

- 변환은 여기서 해서 색깔을 찾으면 된다. 굿
  - https://www.easyrgb.com/en/convert.php#inputFORM

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

<hr>

# Rust_Game_Dev | GlobalYoung(정리중.. 모아보기)

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

<hr>

# Doom 만들면서 게임엔진 이해하기(Rust Code)[[🔝]](#link)
- https://gitlab.com/flukejones/room4doom
  - https://ljones.dev/blog/room4doom-20220529/
