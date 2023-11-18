#version 450 core
#pragma optionNV (unroll all)
#extension GL_NV_gpu_shader5 : enable
#extension GL_NV_shader_atomic_fp16_vector : require

#define PI 3.1415926538
#define APT PI/32.
#define BIAS 0.01
#define beta 2.0

layout (location=0) in vec3 _worldnormal;
layout (location=1) in vec2 texcoord;
layout (location=2) in vec3 worldpos;
layout (location=3) in vec3 _lworldpos;
uniform sampler3D grid;
uniform uint GWIDTH;
uniform vec3 cameraworldpos;
uniform bool enabled;
uniform bool only_occ;
uniform bool enable_indspec;
uniform bool enable_inddiff;
uniform bool enable_dir;
uniform bool write_vox;
uniform layout (rgba16f) coherent image3D wgrid;
uniform sampler2D kd;
uniform sampler2D ks;
uniform float shininess;

out vec4 fragcolor;

float gwidth = float(GWIDTH);
vec3 lworldpos = _lworldpos + vec3(0.5, 0.5, 0.5);
vec3 ldir = normalize(lworldpos - worldpos);
vec3 worldnormal = normalize(_worldnormal);
float bias = 4.0/gwidth;

float radius2miplvl(float radius) {
    return log2(radius*gwidth*2.0);
}

float occlusiontrace() {
    vec3 pos;
    float radius;
    float opacity = 0.;
    float dist = bias;
    radius = dist*tan(APT/2.);

    while (opacity < 1.0) {
        pos = worldpos + dist*ldir + worldnormal*bias*0.0;

        vec3 clamped = clamp(pos, 0.,1.);
        if (dot(pos - lworldpos, pos - lworldpos) < radius*radius) {
            // opacity /= 2.0;
            break;
        }
        if (clamped != pos) {
            opacity = 1.0;;
            break;
        }
        radius = dist*tan(APT/2.);
        vec4 rgba = textureLod(grid, pos, radius2miplvl(radius));
        float newopacity = rgba.a/2.; 
        opacity +=(1.-opacity)* newopacity/dist/beta;
        dist += radius*2/beta;
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
        dist += radius*2/beta;
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
    float directspec = pow(max(dot(viewdir, reflectdir), 0.0), shininess);

    vec3 direct = diffusecolor.rgb*directdiffuse+speccolor.rgb*directspec;

    if (enabled)
    {
        float occlusion = occlusiontrace();
        direct *= (1-occlusion);
    }
    
    if (enabled && !only_occ) {
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

        vec3 clr = vec3(0., 0., 0.);
        if (enable_dir)
            clr += direct;

        if (enable_inddiff)
            clr += inddiff*diffusecolor.rgb;

        vec3 refldir = -reflect(viewdir, worldnormal);
        vec3 spec = vec3(0., 0., 0.);
        spec += (trace(PI/64., refldir).rgb)*1.0;
        if (enable_indspec)
            clr += spec*diffusecolor.rgb;
        
        fragcolor = vec4(clr, diffusecolor.a);
    } else if (!enabled) {
        fragcolor = vec4(direct, diffusecolor.a);
    } else {
        fragcolor = vec4(direct, diffusecolor.a);
    }
    // fragcolor = vec4(viewnormal, 1.0);

    if (write_vox) {
        imageAtomicMax(
            wgrid,
            ivec3(
                (worldpos*gwidth)
            ),
            f16vec4(fragcolor)
        );
    }
}
