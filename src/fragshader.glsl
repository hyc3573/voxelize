#version 450 core

#define GWIDTH 64

in vec4 worldpos;
out vec4 color;

uniform layout (binding=4, rgba32f) image3D grid;

void main() {
    color = vec4(1.0, 1.0, 1.0, 1.0);
    // imageStore(grid, ivec3((worldpos*GWIDTH).xyz), vec4(1.0, 1.0, 1.0, 1.0));
    imageStore(grid, ivec3(0, 0, 0), vec4(1.0, 1.0, 1.0, 1.0));
}
