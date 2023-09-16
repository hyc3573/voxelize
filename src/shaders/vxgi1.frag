#version 450 core
#pragma optionNV (unroll all)

#define LPOS vec3(0., 0., -1.)
#define PI 3.1415926538
#define APT PI/4.
#define BIAS 0.01
#define TQT 2.8284271247

in vec3 normal;
in vec2 texcoord;
in vec3 voxcoord;
uniform sampler3D grid;
uniform uint GWIDTH;

out vec4 color;

float gwidth = float(GWIDTH);

float radius2miplvl(float radius) {
    return log2(radius*gwidth*2.0);
}

void main() { 
    float opacity = 0.;
    vec3 dir = normalize(LPOS - voxcoord);

    float dist = 1.0/gwidth;
    // float dist = 0.001;
    
    vec3 pos;
    // vec3 voxbias = voxcoord + normal*dist;
    float radius;
    while (opacity < 1.)
    {
        pos = voxcoord + dist*dir;

        vec3 clamped = clamp(pos, 0.,1.);
        if (dot(pos - LPOS, pos - LPOS) < radius*radius)
            break;

        if (clamped != pos)
            break;

        radius = dist*tan(APT/2.);
        float newopacity = textureLod(grid, pos, radius2miplvl(radius)).a;
        opacity +=(1.-opacity)* newopacity;
        opacity = min(1., opacity);
        dist += radius*2;
    }

        opacity = step(0.1, opacity);

    color = vec4(vec3(1.0, 1.0, 1.0)*(1.0-opacity/1.), 1.0);

}
