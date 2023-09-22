#version 450 core
#pragma optionNV (unroll all)

#define PI 3.1415926538
#define APT PI/8.
#define BIAS 0.01

in vec3 _worldnormal;
in vec3 _viewnormal;
in vec2 texcoord;
in vec3 worldpos;
in vec3 viewpos;
in vec3 lworldpos;
in vec3 lviewpos;
uniform sampler3D grid;
uniform uint GWIDTH;
uniform vec3 cameraworldpos;
uniform bool enabled;
uniform bool only_occ;
uniform sampler2D tex;

uniform sampler2D kd;
uniform sampler2D ks;

out vec4 fragcolor;

float gwidth = float(GWIDTH);
vec3 ldir = normalize(lworldpos - worldpos);
vec3 worldnormal = normalize(_worldnormal);
vec3 viewnormal = normalize(_viewnormal);

float radius2miplvl(float radius) {
    return log2(radius*gwidth*2.0);
}

float occlusiontrace() {
    vec3 pos;
    float radius;
    float opacity = 0.;
    float dist = 1.5/gwidth;
    radius = dist*tan(APT/2.);

    while (opacity < 1.0) {
        pos = worldpos + dist*ldir;

        vec3 clamped = clamp(pos, 0.,1.);
        if (dot(pos - lworldpos, pos - lworldpos) < radius*radius) {
            opacity /= 2.0;
            break;
        }
        if (clamped != pos) {
            // this means that lpos is outside of NDC boundary
            break;
        }
        radius = dist*tan(APT/2.);
        vec4 rgba = textureLod(grid, pos, radius2miplvl(radius));
        float newopacity = rgba.a; 
        opacity +=(1.-opacity)* newopacity/dist;
        dist += radius*2;
    }
    opacity = min(1., opacity);

    return opacity;
}

vec4 trace(float apt, vec3 dir) {
    vec3 pos;
    vec4 color = vec4(0., 0., 0., 0.);
    float radius;
    float dist = 1.0/gwidth;

    while (color.a < 1.0) {
        pos = worldpos + dist*dir;

        vec3 clamped = clamp(pos, 0., 1.);

        if (clamped != pos)
            break;

        radius = dist*tan(apt/2);
        vec4 rgba = textureLod(grid, pos, radius2miplvl(radius));
        color = vec4(
            color.a*color.rgb + (1.0-color.a)*rgba.a*rgba.rgb,
            color.a+(1.0-color.a)*rgba.a
        );
        dist += radius*2;
    }
    color.a = 1.0;
    
    return color;
}

void main() { 
    vec4 diffusecolor = texture(kd, texcoord);
    vec4 speccolor = texture(ks, texcoord);
    
    float directdiffuse = max(dot(ldir, worldnormal), 0.0);

    vec3 viewdir = normalize(cameraworldpos - worldpos);
    vec3 reflectdir = reflect(-ldir, worldnormal);
    float directspec = pow(max(dot(viewdir, reflectdir), 0.0), 32);

    vec3 direct = diffusecolor.rgb*directdiffuse+speccolor.rgb*directspec;
    
    if (enabled && !only_occ) {
        float occlusion = occlusiontrace();
        
        vec3 N = worldnormal;
        vec3 T = cross(N, vec3(0.0f, 1.0f, 0.0f));
        vec3 B = cross(T, N);

        vec3 inddiff = vec3(0., 0., 0.);

        vec3 dir = worldnormal;
        inddiff += trace(PI/3., dir).rgb;
        dir = 0.7071f * N + 0.7071f * T;
        inddiff += trace(PI/3., dir).rgb;
        dir = 0.7071f * N + 0.7071f * (0.309f * T + 0.951f * B);
        inddiff += trace(PI/3., dir).rgb;
        dir = 0.7071f * N + 0.7071f * (-0.809f * T + 0.588f * B);
        inddiff += trace(PI/3., dir).rgb;
        dir = 0.7071f * N - 0.7071f * (-0.809f * T - 0.588f * B);
        inddiff += trace(PI/3., dir).rgb;
        dir = 0.7071f * N - 0.7071f * (0.309f * T - 0.951f * B);
        inddiff += trace(PI/3., dir).rgb;

        vec3 clr = direct*(1.-occlusion);
        clr += inddiff/6.;

        vec3 refldir = -reflect(viewdir, worldnormal);
        vec3 spec = vec3(0., 0., 0.);
        spec += trace(PI/6., refldir).rgb;
        clr += spec;
        
        clr /= 4;
        
        fragcolor = vec4(clr, diffusecolor.a);
    } else if (!enabled) {
        fragcolor = vec4(direct, diffusecolor.a);
    } else {
        float occlusion = occlusiontrace();
        fragcolor = vec4(direct, diffusecolor.a);
    }
    // fragcolor = vec4(viewnormal, 1.0);
}
