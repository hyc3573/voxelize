#version 450 core

uniform layout (rgba32f) writeonly image3D grid;
uniform int depth;

void main() {
    imageStore(grid, ivec3(gl_FragCoord.xy, depth), vec4(0., 0., 0., 0.));
}
