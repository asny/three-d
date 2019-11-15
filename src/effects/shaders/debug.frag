
uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;

uniform int type;

uniform mat4 projectionInverse;
uniform mat4 viewInverse;

in vec2 uv;

layout (location = 0) out vec4 color;

uniform float zNear;
uniform float zFar;

float linear_depth(float z)
{
    float n = 0.1; // camera z near
    float f = 10.0; // camera z far
    return (2.0 * n) / (f + n - z * (f - n));
}

vec3 WorldPosFromDepth(float depth, vec2 uv) {
    float z = depth * 2.0 - 1.0;

    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, z, 1.0);
    vec4 viewSpacePosition = projectionInverse * clipSpacePosition;

    // Perspective division
    viewSpacePosition /= viewSpacePosition.w;

    return (viewInverse * viewSpacePosition).xyz;
}

void main()
{
    if(type == 0) // Position
    {
        float depth = texture(depthMap, vec3(uv, 0)).x;
        vec3 pos = WorldPosFromDepth(depth, uv);
        color = vec4(pos, 1.);
    }
    else if(type == 1) // Normal
    {
        color = vec4(texture(gbuffer, vec3(uv, 1)).xyz, 1.);
    }
    else if(type == 2) // Color
    {
        color = vec4(texture(gbuffer, vec3(uv, 0)).xyz, 1.);
    }
    else if(type == 3) // Depth
    {
        float depth = linear_depth(texture(depthMap, vec3(uv, 0)).x);
        color = vec4(depth, depth, depth, 1.);
    }
    else if(type == 4) // Diffuse
    {
        float val = texture(gbuffer, vec3(uv, 2)).x;
        color = vec4(val, val, val, 1.);
    }
    else if(type == 5) // Specular
    {
        float val = texture(gbuffer, vec3(uv, 2)).y;
        color = vec4(val, val, val, 1.);
    }
    else if(type == 6) // Specular power
    {
        float val = texture(gbuffer, vec3(uv, 2)).z;
        color = vec4(val, val, val, 1.);
    }
    else {
        color = vec4(0., 0., 0., 0.);
    }
}