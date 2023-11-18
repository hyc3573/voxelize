#version 450 core
#extension GL_NV_gpu_shader5 : enable

uniform layout (rgba16f) coherent image3D grid;
uniform int depth;

void main() {
    imageStore(grid, ivec3(gl_FragCoord.xy, depth), f16vec4(0., 0., 0., 0.));
}
