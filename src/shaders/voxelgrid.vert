# version 450 core

in vec2 pos;
uniform int depth;
uniform uint GWIDTH;
uniform mat4 matrix;

out vec3 texcoord;

void main()
{
    gl_Position = matrix*vec4(pos, depth/float(GWIDTH)*2.0-1.0, 1.0);

    texcoord = vec3((pos+vec2(1., 1.))/2., float(depth)/float(GWIDTH));
}
