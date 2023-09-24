#version 450 core

layout (triangles) in;
layout (triangle_strip, max_vertices = 3) out;

in vec3 position[];
in vec3 normal[];
in vec2 texcoord[];
out vec3 clippos;
out vec3 nor;                
out vec2 tex;                  

uniform uint GWIDTH;

void main() {
    // compute dominant axis
    vec4 A = gl_in[0].gl_Position - gl_in[1].gl_Position;
    vec4 B = gl_in[1].gl_Position - gl_in[2].gl_Position;

    vec3 N = abs(cross(A.xyz, B.xyz));

    vec3 newpos[3] = {
        vec3(0.), vec3(0.), vec3(0.)
    };

    vec4 G = (gl_in[0].gl_Position + gl_in[1].gl_Position + gl_in[2].gl_Position)/3.;

    for (int i=0;i<3;i++)
    {
        vec3 P = gl_in[i].gl_Position.xyz;
        if (N.z > N.x && N.z > N.y)
        {
            newpos[i] = vec3(P.x, P.y, P.z);
            
        }
        else if (N.x > N.z && N.x > N.y)
        {
            newpos[i] = vec3(P.y, P.z, P.x);
        }
        else
        {
            newpos[i] = vec3(P.x, P.z, P.y);
        }

        vec3 correction = normalize(G.xyz - newpos[i])*(5./float(GWIDTH));

        gl_Position = vec4(newpos[i]+correction, 1.0);
        clippos = position[i];
        tex = texcoord[i];
        nor = normal[i];
        EmitVertex();
    }

    EndPrimitive();
}
