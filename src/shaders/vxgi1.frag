#version 450 core

#define LPOS vec3(0., 10., 0.)
#define PI 3.1415926538
#define APT PI/10 
#define BIAS 0.01

in vec3 normal;
in vec2 texcoord;
in vec3 voxcoord;
uniform sampler3D grid;
uniform uint GWIDTH;

out vec4 color;

float radius(float miplvl) {
    return 1./GWIDTH*pow(2, miplvl);
}

void main() {
    float opacity = 0.;
    vec3 dir = normalize(LPOS - voxcoord);
    vec3 pos = voxcoord + dir*BIAS;
    float steplen = tan(APT)/64.;
    float miplvl = 0;

    int step = 0;
    while (length(pos - LPOS) > radius(miplvl) || opacity >= 1. || step > 2)
    {
        opacity += textureLod(grid, pos, miplvl).a;
        float rad = radius(miplvl);
        pos += rad;
        miplvl += log2(rad*(1+tan(APT)));
        step++;
    }

    color = vec4(vec3(1.0, 1.0, 1.0)*(1.-opacity), 1.0);
}
