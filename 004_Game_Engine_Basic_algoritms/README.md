# 여기에 대부분 정리중

- https://github.com/YoungHaKim7/Algorithm_Training

<hr>

# Deriving 3D Rigid Body Physics and implementing it in C/C++ (with intuitions) | blackedout01
- https://youtu.be/4r_EvmPKOvY?si=BUS4OK2FpS-J5Lne

# Devlog | blackedout01 모아보기(XPBD설명 C++코드로 해줌.)
-https://youtube.com/playlist?list=PLwMZtAEBQ8ZywWPf6twbspmYzGg0Fr2DJ&si=jyOANeioFu8m8pVa

<hr>

# quaternion to 4*4 matrix ?
- https://www.reddit.com/r/rust_gamedev/comments/obk3w9/quaternion_to_44_matrix/

# 입체 투영으로 쿼터니언(4d 숫자) 시각화| 3Blue1Brown
- https://youtu.be/d4EgbgTm0Bg?si=yTm8-X8ARBHF8cTU

# RayTracing에 쓰는 구나(햇빛반사 &  그림자 같은거)(C++지만 배울게 많다)
- https://github.com/TheCherno/RayTracing


# 3D-Fractal-Rendering(실사 만들 때 굿)
- Exploring generation and visualization of various techniques that involve fractal rendering. 
  - https://github.com/adrialfonso/fractal-rendering

<hr>

# (러스트 라이브러리) quaternion
- https://docs.rs/quaternion/latest/quaternion/

- https://docs.rs/cgmath/latest/cgmath/struct.Quaternion.html


# A basic quaternion library written in C(역시 근본은 C언어)
- https://github.com/MartinWeigel/Quaternion

<hr>

# (Rust코드glam-rs)A simple and fast linear algebra library for games and graphics
- https://github.com/bitshifter/glam-rs


# Designing a Physics Engine in 5 minutes | Winterdev

- https://youtu.be/-_IspRG548E?si=Txo6_TEeTP5Kifn-


# How to rotate 2D image in 3D space using a quaternion
- https://gamedev.stackexchange.com/questions/204878/how-to-rotate-2d-image-in-3d-space-using-a-quaternion

<hr>

# Polygon mesh
- https://en.wikipedia.org/wiki/Polygon_mesh

<h2><span class="mw-headline" id="Elements">Elements</span><span class="mw-editsection"><span class="mw-editsection-bracket">[<span class="mw-editsection-bracket">]</span></span></h2>
<p><span class="mw-default-size" typeof="mw:File"><a href="/wiki/File:Mesh_overview.svg" class="mw-file-description" title="Elements of polygonal mesh modeling."><img alt="Elements of polygonal mesh modeling." src="view-source:https://en.wikipedia.org/wiki/upload.wikimedia.org/wikipedia/commons/thumb/6/6d/Mesh_overview.svg/720px-Mesh_overview.svg.png" decoding="async" width="720" height="271" class="mw-file-element" srcset="view-source:https://en.wikipedia.org/wiki/upload.wikimedia.org/wikipedia/commons/thumb/6/6d/Mesh_overview.svg/1080px-Mesh_overview.svg.png 1.5x, https://en.wikipedia.org/wiki/upload.wikimedia.org/wikipedia/commons/thumb/6/6d/Mesh_overview.svg/1440px-Mesh_overview.svg.png 2x" data-file-width="720" data-file-height="271" /></a></span>
</p><p>Objects created with polygon meshes must store different types of elements. These include vertices, edges, faces, polygons and surfaces. In many applications, only vertices, edges and either faces or polygons are stored. A renderer may support only 3-sided faces, so polygons must be constructed of many of these, as shown above. However, many renderers either support quads and higher-sided polygons, or are able to convert polygons to triangles on the fly, making it unnecessary to store a mesh in a <a href="view-source:https://en.wikipedia.org/wiki/Surface_triangulation" title="Surface triangulation">triangulated</a> form. 
</p>
