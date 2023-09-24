#version 450 core

in vec3 pos;
in vec3 nor;
in vec2 tex;

layout (location=0) out vec3 _worldnormal;
layout (location=1) out vec2 texcoord;
layout (location=2) out vec3 worldpos;
layout (location=3) out vec3 lworldpos;

uniform mat4 VNM;
uniform mat4 RNM;
uniform mat4 M;
uniform mat4 V;
uniform mat4 P;
uniform mat4 VP;
uniform mat4 VV;
uniform vec3 lpos;

vec3 LPOS = vec3(lpos.xy, lpos.z);

void main()
{
    gl_Position = P*V*M*vec4(pos, 1.0);
    _worldnormal = mat3(VNM)*nor;
    texcoord = tex;
    worldpos = (VP*VV*M*vec4(pos, 1.0)+vec4(1.0, 1.0, 1.0, 0.0)).xyz/2.0;
    lworldpos = LPOS;
}
