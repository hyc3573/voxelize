d#version 450 core

in vec3 tex;
out vec4 color;
uniform sampler3D grid;

void main{
    vec4 temp = texture(grid, tex);
    if (temp.a < 0.1)
        discard;

    color = vec4(temp.rgb, 1.0)
}
