#version 450 core

in vec3 pos;
in vec3 nor;
in vec2 tex;
// 입력 매개변수 선언

out vec3 position;
out vec3 normal;
out vec3 texcoord;
// 출력 매개변수 선언

uniform mat4 M;
uniform mat4 V;
uniform mat4 P;
// 유니폼 매개변수 선언

void main()
{
    // gl_Position = P*V*M*vec4(position, 1.0);
    gl_Position = P*V*M*vec4(pos, 1.0); // 위치 벡터 변환
    position = gl_Position.xyz; // 변환된 위치 벡터를 Geometry Shader로 전송
}
