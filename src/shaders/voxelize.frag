#version 450 core
#extension GL_NV_shader_atomic_fp16_vector : require
#extension GL_NV_gpu_shader5 : enable

in vec3 clippos; // Geometry Shader에서 전달받은 위치 벡터
in vec3 nor;
in vec2 tex;

uniform layout (rgba16f) coherent image3D grid; // 3D 텍스쳐
uniform uint GWIDTH; // 텍스쳐 너비
uniform sampler2D image;
uniform vec3 lpos;

vec3 LPOS = vec3(lpos.xy, -lpos.z);

void main() {
    vec4 diffcolor = texture(image, tex);

    vec3 ldir = normalize(LPOS-clippos);

    float diff = max(dot(ldir, nor), 1.0);

    vec3 fragcolor = diff*diffcolor.rgb;

    imageAtomicMax(
        grid,
        ivec3(
            (clippos+vec3(1., 1., 1.))*float(GWIDTH)/2.
        ),
        f16vec4(fragcolor,diffcolor.a)
    );
    // 3D 텍스쳐에 Fragment 정보를 씀 (미완성)
}
