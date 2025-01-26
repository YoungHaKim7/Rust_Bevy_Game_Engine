# 여기에 정리 중 ...(shader언어를 알아야함)
- https://github.com/YoungHaKim7/SDL3_Rust_Game_Training/tree/main/08_HLSL_Shader_Fundamentals

<hr />

# I am trying to rotate a flat 2D image around a 2D origin in 3D space. 
- The problem I am having is that the image becomes stretched after the rotation. I am using the WGSL from this tutorial, but with the camera logic removed. The clip-coordinates have to be a 3d vector + w.
  - https://sotrh.github.io/learn-wgpu/beginner/tutorial7-instancing/#the-instance-buffer
