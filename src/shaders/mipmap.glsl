#version 450 core

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout (rgba16f) writeonly image3D resultimg;
uniform sampler3D sampler;
uniform int lod;

void main()
{
    ivec3 resultcoord = ivec3(gl_GlobalInvocationID);
    ivec3 imgsize = imageSize(resultimg);
    vec3 mipcoord = resultcoord / imgsize;
    vec4 result = vec4(0., 0., 0., 0.);
    result += textureLod(sampler, mipcoord, lod);
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(1, 0, 0));
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(0, 1, 0));
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(1, 1, 0));
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(0, 0, 1));
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(1, 0, 1));
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(0, 1, 1));
    result += textureLodOffset(sampler, mipcoord, lod, ivec3(1, 1, 1));

    result /= 8.0;
    imageStore(resultimg, resultcoord, result);
}
