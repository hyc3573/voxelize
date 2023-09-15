#version 450 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec3 position[];
in vec3 normal[];
in vec2 texcoord[];
out vec3 clippos;
out vec3 nor;                
out vec2 tex;                  

void main() {
    // compute dominant axis
    vec4 A = gl_in[0].gl_Position - gl_in[1].gl_Position;
    vec4 B = gl_in[1].gl_Position - gl_in[2].gl_Position;

    vec3 N = normalize(cross(A.xyz, B.xyz));
    float x=abs(N.x), y=abs(N.y), z=abs(N.z);

    vec3 newpos[3] = {
        vec3(0.), vec3(0.), vec3(0.)
    };

    vec4 G = (gl_in[0].gl_Position + gl_in[1].gl_Position + gl_in[2].gl_Position)/3.;

    // dominant axis: x
    if (x > y && x > z)
    {
        for (int i=0;i<3;i++)
        {
            newpos[i] = gl_in[i].gl_Position.yzx;
        }
    }
    else if (y > x && y > z)
    {
        for (int i=0;i<3;i++)
        {
            newpos[i] = gl_in[i].gl_Position.xzy;
        }
    }
    else
    {
        for (int i=0;i<3;i++)
        {
            newpos[i] = gl_in[i].gl_Position.xyz;
        }
    }
    
    for (int i=0;i<3;i++)
    {
        gl_Position = vec4(newpos[i], 1.0);
        clippos = position[i];
        tex = texcoord[i];
        nor = normal[i];
        EmitVertex();
    }

    EndPrimitive();
}
