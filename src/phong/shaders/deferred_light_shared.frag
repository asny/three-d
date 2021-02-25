
struct Surface
{
    vec3 position;
    vec3 normal;
    vec3 color;
    float diffuse_intensity;
    float specular_intensity;
    float specular_power;
};

uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;
uniform mat4 viewProjectionInverse;

in vec2 uv;

vec3 WorldPosFromDepth(float depth, vec2 uv) {
    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth * 2.0 - 1.0, 1.0);
    vec4 position = viewProjectionInverse * clipSpacePosition;
    return position.xyz / position.w;
}

float get_surface_depth()
{
    float depth = texture(depthMap, vec3(uv,0)).r;
    if(depth > 0.99999)
    {
        discard;
    }
    gl_FragDepth = depth;
   	return depth;
}

vec3 get_surface_color()
{
    get_surface_depth();
   	return texture(gbuffer, vec3(uv, 0)).rgb;
}

Surface get_surface()
{
    float depth = get_surface_depth();
   	vec4 c = texture(gbuffer, vec3(uv, 0));
    vec3 surface_color = c.rgb;
    vec3 position = WorldPosFromDepth(depth, uv);
    vec4 n = texture(gbuffer, vec3(uv, 1));
    vec3 normal = normalize(n.xyz*2.0 - 1.0);
    float diffuse_intensity = c.w;
    int t = int(floor(n.w*255.0));
    float specular_intensity = float(t & 15) / 15.0;
    float specular_power = 2.0 * float((t & 240) >> 4);

    return Surface(position, normal, surface_color, diffuse_intensity, specular_intensity, specular_power);
}
