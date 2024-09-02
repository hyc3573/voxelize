#version 450 core

layout (triangles) in;
layout (triangle_strip, max_vertices=3) out;

layout (location=0) in vec3 _worldnormal[];
layout (location=1) in vec2 texcoord[];
layout (location=2) in vec3 worldpos[];
layout (location=3) in vec3 lworldpos[];

layout (location=0) out vec3 wrdnrmOut;
layout (location=1) out vec2 texcrdOut;
layout (location=2) out vec3 wrdposOut;
layout (location=3) out vec3 lwdposOut;

void main() {
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
            newpos[i] = vec3(P.x, P.y, 0.f);
            
        }
        else if (N.x > N.z && N.x > N.y)
        {
            newpos[i] = vec3(P.y, P.z, 0.f);
        }
        else
        {
            newpos[i] = vec3(P.x, P.z, 0.f);
        }
    
        gl_Position = vec4(newpos[i], 1.0);
        wrdnrmOut = _worldnormal[i];
        texcrdOut = texcoord[i];
        wrdposOut = worldpos[i];
        lwdposOut = lworldpos[i];
        EmitVertex();
    }

    EndPrimitive();

}
