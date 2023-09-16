#version 450 core

in vec3 pos;
uniform uint GWIDTH;
uniform mat4 M;
uniform mat4 V;
uniform mat4 P;
uniform mat4 VP;
uniform mat4 VV;

out vec3 texcoord;

void main()
{
    vec3 n_pos = pos/float(GWIDTH/2) - vec3(1., 1., 1.);
    gl_Position = vec4(n_pos, 1.0);
    texcoord = n_pos;
}
