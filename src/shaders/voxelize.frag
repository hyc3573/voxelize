#version 450 core

in vec3 clippos; // Geometry Shader에서 전달받은 위치 벡터

uniform layout (rgba32f) writeonly image3D grid; // 3D 텍스쳐
uniform uint GWIDTH; // 텍스쳐 너비

void main() {

    imageStore(
        grid,
        ivec3(
            (clippos+vec3(1., 1., 1.))*float(GWIDTH)/2.
        ),
        vec4(1.0, 1.0, 1.0, 1.0)
    );
    // 3D 텍스쳐에 Fragment 정보를 씀 (미완성)
}
