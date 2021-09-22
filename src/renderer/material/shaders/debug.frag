
uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;

uniform int type;

uniform mat4 viewProjectionInverse;

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
    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth * 2.0 - 1.0, 1.0);
    vec4 position = viewProjectionInverse * clipSpacePosition;
    return position.xyz / position.w;
}

void main()
{
    float depth = texture(depthMap, vec3(uv,0)).r;
    if(depth > 0.99999)
    {
        discard;
    }
    if(type == 0) // Position
    {
        float depth = texture(depthMap, vec3(uv, 0)).x;
        vec3 pos = WorldPosFromDepth(depth, uv);
        color = vec4(pos, 1.);
    }
    else if(type == 1) // Normal
    {
        vec4 n = texture(gbuffer, vec3(uv, 1));
        vec2 n2 = n.xy*2.0 - 1.0;
        float z = 1.0 - n2.x * n2.x - n2.y * n2.y;
        if (z > 0.0001) {
            z = sqrt(z);
        }
        vec3 normal = normalize(vec3(n2.x, n2.y, z));
        color = vec4(normal * 0.5 + 0.5, 1.);
    }
    else if(type == 2) // Color
    {
        color = vec4(texture(gbuffer, vec3(uv, 0)).xyz, 1.);
    }
    else if(type == 3) // Depth
    {
        depth = linear_depth(depth);
        color = vec4(depth, depth, depth, 1.);
    }
    else if(type == 4) // Diffuse
    {
        float val = texture(gbuffer, vec3(uv, 0)).w;
        color = vec4(val, val, val, 1.);
    }
    else if(type == 5) // Specular
    {
        float nw = texture(gbuffer, vec3(uv, 1)).w;
        int t = int(floor(nw*255.0));
        float val = float(t & 15) / 15.0;
        color = vec4(val, val, val, 1.);
    }
    else if(type == 6) // Specular power
    {
        float nw = texture(gbuffer, vec3(uv, 1)).w;
        int t = int(floor(nw*255.0));
        float val = 2.0 * float((t & 240) >> 4);
        val /= 30.0;
        color = vec4(val, val, val, 1.);
    }
    else {
        color = vec4(0., 0., 0., 0.);
    }
}