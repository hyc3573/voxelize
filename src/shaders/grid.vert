#version 450 core

in vec3 pos;
uniform uint GWIDTH;
uniform mat4 matrix;

out vec3 texcoord;

void main()
{
    vec3 n_pos = pos/float(GWIDTH/2) + vec3(1., 1., 1.);
    texcoord = pos/float(GWIDTH);
    gl_Position = vec4(pos, 1.0);
}
