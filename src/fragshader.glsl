#version 450 core

in vec3 clippos;
// out vec4 color;

uniform layout (rgba32f) writeonly image3D grid;
uniform uint GWIDTH;

void main() {
    // color = vec4(clippos, 1.0);

    imageStore(grid, ivec3((clippos+vec3(1., 1., 1.))*float(GWIDTH)/2.), vec4(1.0, 1.0, 1.0, 1.0));
    // imageStore(grid, ivec3(0, 0, 0), vec4(1.0, 1.0, 1.0, 1.0));
}
