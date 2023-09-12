#version 450 core

layout (triangles) in;
layout (triangle_strip, max_vertices = ) out;

uniform uint GWIDTH;
in vec3 texcoord;
out vec3 tex;                   

void main()
{
    float cubewidth = 2./float(GWIDTH);


}
