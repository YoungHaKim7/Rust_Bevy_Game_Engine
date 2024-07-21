# link

- [물리엔진 찾기](#그-외에-물리엔진-많다)

- 물리엔진
  - [avian2D & 3D](##avian2d--3d)
  - [rapier](#rust-bevy-물리엔진은-지금)

<hr>

- Rendering
  - [bevy_vector shapes](#bevy-vector-shapes)


- Camera
  - [bevy-3d-third-person-tutorial--part-4--third-person-camera--thedevblog](#bevy-3d-third-person-tutorial--part-4--third-person-camera--thedevblog) 

<hr>

# 2D and 3D physics engine based on Extended Position Based Dynamics for Bevy.[[🔝]](#link)
- 이걸로 변경 예정인듯 https://crates.io/crates/avian/0.0.0
  - https://github.com/Jondolf/bevy_xpbd
    - https://github.com/Jondolf/bevy_xpbd/issues/346
      - XPBD가 NVIDIA저작권 문제로 대규모 업데이트 예정인 프로젝트 역시 저작권이 걸리네 ㅠㅠ

# avian2D & 3D[[🔝]](#link)
- https://docs.rs/avian2d/latest/avian2d/
  - https://github.com/Jondolf/avian

- 2D Examples
  - https://github.com/Jondolf/avian/tree/main/crates/avian2d/examples

- 3D Examples
  - https://github.com/Jondolf/avian/tree/main/crates/avian3d/examples

<hr>

# Rust Bevy 물리엔진은 지금 [[🔝]](#link)
- https://rapier.rs/

<hr>

# 그 외에 물리엔진 많다[[🔝]](#link)
- https://arewegameyet.rs/ecosystem/physics/

# Bevy Vector Shapes[[🔝]](#link)
- A library for rendering vector shapes using the Bevy game engine
  - https://github.com/james-j-obrien/bevy_vector_shapes

<hr>

# Bevy 3d Third Person Tutorial | Part 4 | Third Person Camera | TheDevBlog[[🔝]](#link)
- https://youtu.be/__8rhqHc82I?si=lpov6hP1S1FSjJyW

# 멀티 플레이 github
- BrianWiz/bevy_fps
  - https://github.com/BrianWiz/bevy_fps

- Hi there I'm looking for some reviewers, particularly people who are interested and knowledgeable with multiplayer design:

This is a very early implementation and there's much more to come, I'm just laying the groundwork. It currently features:
- Networking using Quinnet
- Move and slide character physics with Avian3d, just a basic sphere that can fly for now
- Diffing, only send the values that change between the client's last acked snapshot and the current snapshot
- Extrapolation on *your own* character for players with FPS above the tickrate
- Predicated server authenticated movement, with smooth correction
- Client
- Dedicated Server

What kind of reviewing I'm looking for right now:
- Opinions on overall project structure, keeping in mind that I will also want to support Listen servers as well
- Opinions on overall code cleanliness
- Opinions on doing things the bevy way, and any places where I can make some gains

Next goals:
- Weapons and death!
- Standard character movement with step-up mechanics

Goal of the project:
- Mostly to have fun
- Educate myself, and others on creating multiplayer games
- To show off a bit with my friends at <#747940465936040017>, share ideas (this is mainly why I didn't go with Replicon or Lightyear, as there may be something I do differently they can learn from. It's also a good thing educationally to understand what these higher-level crates actually do)
- A feature-complete arena shooter, but with minimal graphics
- Eventually to make a blog post, anyone helping out in any capacity will be credited

<@263123021336805376> tl;dr this is just a very early edition but I figured I'd tag you since you've shown the most interest. There's no projectiles or any shooting yet.
https://github.com/BrianWiz/bevy_fps