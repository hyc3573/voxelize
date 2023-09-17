#version 450 core
#define LPOS vec3(0., 0., -1.)

in vec3 clippos; // Geometry Shader에서 전달받은 위치 벡터
in vec3 nor;
in vec2 tex;

uniform layout (rgba32f) writeonly image3D grid; // 3D 텍스쳐
uniform uint GWIDTH; // 텍스쳐 너비

void main() {
    vec3 ldir = normalize(LPOS-clippos);

    float diff = max(dot(ldir, nor), 1.0);

    vec3 fragcolor = diff*vec3(1., 1., 1.);

    imageStore(
        grid,
        ivec3(
            (clippos+vec3(1., 1., 1.))*float(GWIDTH)/2.
        ),
        vec4(fragcolor,1.)
    );

    // 3D 텍스쳐에 Fragment 정보를 씀 (미완성)
}
