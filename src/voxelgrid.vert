# version 450 core

in vec2 pos;
uniform int depth;

out vec3 texcoord;

#define GWIDTH 64.0

void main()
{
    gl_Position = vec4(pos, 0.0, 1.0);

    texcoord = vec3((pos+vec2(1., 1.))/2., float(depth)/GWIDTH);
}
