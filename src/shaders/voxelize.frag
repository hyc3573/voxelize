#version 450 core

in vec3 clippos; // Geometry Shader에서 전달받은 위치 벡터
in vec3 nor;
in vec2 tex;

uniform layout (rgba32f) writeonly image3D grid; // 3D 텍스쳐
uniform uint GWIDTH; // 텍스쳐 너비
uniform sampler2D image;
uniform vec3 lpos;
uniform float mul;

vec3 LPOS = vec3(lpos.xy, -lpos.z);

void main() {
    // 표면 색상
    vec4 diffcolor = texture(image, tex);

    // 빛 방향
    vec3 ldir = normalize(LPOS-clippos);

    // 단위면적당 빛으로 인한 에너지 입사량은 입사각의 코사인에 비례
    float diff = max(dot(ldir, nor), 1.0);

    // 최종 색상
    vec3 fragcolor = diff*diffcolor.rgb;

    // 복셀 그리드에 저장
    imageStore(
        grid,
        ivec3(
            (clippos+vec3(1., 1., 1.))*float(GWIDTH)/2.*mul
        ),
        vec4(fragcolor,diffcolor.a)*mul
    );

}
