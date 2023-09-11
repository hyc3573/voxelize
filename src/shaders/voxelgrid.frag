#version 450 core

in vec3 texcoord;
out vec4 color;
uniform sampler3D grid;

void main() {
    vec4 temp = texture(grid, texcoord);
    if (temp.a < 0.1)
        discard;
    color = vec4(temp.rgb*texcoord.z, 1.0);
}
