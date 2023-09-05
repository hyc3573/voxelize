#version 450 core

in vec3 position;
in vec3 normal;
uniform mat4 matrix;

// uniform mat4 M;
// uniform mat4 V;
// uniform mat4 P;

void main()
{
    // gl_Position = P*V*M*vec4(position, 1.0);
    gl_Position = matrix*vec4(position, 1.0);
}
