#version 450 core

layout (points) in;
layout (triangle_strip, max_vertices = 36) out;
in vec3[] texcoord;
out vec3 tex;

uniform uint GWIDTH;

void main()
{
    vec3 ps[] = {
        vec3(0., 0., 0.),
        vec3(1., 0., 0.),
        vec3(0., 1., 0.),
        vec3(1., 1., 0.),
        vec3(0., 0., 1.),
        vec3(1., 0., 1.),
        vec3(0., 1., 1.),
        vec3(1., 1., 1.)
    };

    uint is[] = {
        0, 1, 2,
        2, 3, 1,
        5, 6, 7,
        5, 4, 6,
        1, 5, 4,
        1, 0, 4,
        3, 7, 6,
        3, 2, 6,
        0, 4, 6,
        0, 2, 6,
        1, 5, 4,
        1, 0, 4
    };

    float width = 2./float(GWIDTH);

    for (i=0;i<36;i+=3)    
        for (int j=0;j<3;j++)
        {
            tex = texcoord[0];
            vec4 vert = vec4(ps[is[i]+j], 0.);
            gl_Position = vec4(gl_in[0]+vert*width);
        }
        EmitVertex();
    }
    EndPrimitive();
}
