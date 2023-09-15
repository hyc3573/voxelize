#version 450 core

in vec3 pos;
in vec3 nor;
in vec2 tex;

out vec3 normal;
out vec2 texcoord;
out vec3 voxcoord;

uniform mat3 NM;
uniform mat4 M;
uniform mat4 V;
uniform mat4 P;
uniform mat4 VP;

void main()
{
    gl_Position = P*V*M*vec4(pos, 1.0);
    normal = NM*nor;
    texcoord = tex;
    voxcoord = (VP*V*M*vec4(pos, 1.0)+vec4(1.0, 1.0, 1.0, 0.0)).xyz/2.0;
}
