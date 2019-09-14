
uniform sampler2DArray gbuffer;
uniform sampler2D depthMap;

uniform int type;

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

void main()
{
    if(type == 0) // Position
    {
        color = vec4(texture(gbuffer, vec3(uv, 1)).xyz, 1.);
    }
    else if(type == 1) // Normal
    {
        color = vec4(0.5 * normalize(texture(gbuffer, vec3(uv, 2)).xyz) + 0.5, 1.);
    }
    else if(type == 2) // Color
    {
        color = vec4(texture(gbuffer, vec3(uv, 0)).xyz, 1.);
    }
    else if(type == 3) // Depth
    {
        float depth = linear_depth(texture(depthMap, uv).x);
        color = vec4(depth, depth, depth, 1.);
    }
    else {
        color = vec4(0., 0., 0., 0.);
    }
}