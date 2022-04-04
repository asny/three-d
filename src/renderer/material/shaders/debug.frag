
uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;

uniform int type;

uniform mat4 viewProjectionInverse;

in vec2 uv;

layout (location = 0) out vec4 color;

uniform float zNear;
uniform float zFar;
uniform vec3 cameraPosition;

void main()
{
    float depth = texture(depthMap, vec3(uv,0)).r;
    if(depth > 0.99999)
    {
        discard;
    }
    if(type == 0) // Position
    {
        vec3 pos = world_pos_from_depth(viewProjectionInverse, depth, uv);
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
        vec3 normal = normalize(vec3(n2.x, n2.y, (int(floor(n.z * 255.0)) & 128) == 128 ? z: -z));
        color = vec4(normal * 0.5 + 0.5, 1.);
    }
    else if(type == 2) // Color
    {
        color = vec4(srgb_from_rgb(texture(gbuffer, vec3(uv, 0)).xyz), 1.);
    }
    else if(type == 3) // Depth
    {
        vec3 pos = world_pos_from_depth(viewProjectionInverse, depth, uv);
        float dist = (distance(pos, cameraPosition) - zNear) / (zFar - zNear);
        color = vec4(dist, dist, dist, 1.);
    }
    else if(type == 4) // ORM
    {
        vec4 c = texture(gbuffer, vec3(uv, 0));
        float metallic = c.w;
        vec4 n = texture(gbuffer, vec3(uv, 1));
        float roughness = n.w;
        float occlusion = float(int(floor(n.z * 255.0)) & 127) / 127.0;
        color = vec4(occlusion, roughness, metallic, 1.0);
    }
    else if(type == 5) // UV
    {
        color = vec4(uv, 0., 1.);
    }
    else {
        color = vec4(0., 0., 0., 0.);
    }
}