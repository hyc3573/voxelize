#version 450 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec3 position[];
out vec3 clippos;

void main() {
    for (int i=0;i<3;i++)
    {
        gl_Position = gl_in[i].gl_Position;
        clippos = position[i];
        EmitVertex();
    }

    EndPrimitive();
}
