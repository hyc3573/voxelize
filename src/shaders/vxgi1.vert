#version 450 core

#define LPOS vec3(0., 0., 1.)

in vec3 pos;
in vec3 nor;
in vec2 tex;

out vec3 _worldnormal;
out vec3 _viewnormal;
out vec2 texcoord;
out vec3 worldpos;
out vec3 viewpos;
out vec3 lworldpos;
out vec3 lviewpos;

uniform mat4 VNM;
uniform mat4 RNM;
uniform mat4 M;
uniform mat4 V;
uniform mat4 P;
uniform mat4 VP;
uniform mat4 VV;

void main()
{
    viewpos = (V*M*vec4(pos, 1.0)).xyz;
    gl_Position = P*V*M*vec4(pos, 1.0);
    _worldnormal = mat3(VNM)*nor;
    _viewnormal = mat3(RNM)*nor;
    texcoord = tex;
    worldpos = (VP*VV*M*vec4(pos, 1.0)+vec4(1.0, 1.0, 1.0, 0.0)).xyz/2.0;
    lworldpos = LPOS;
    lviewpos = (V*vec4(LPOS, 1.0)).xyz;
}
