
in vec2 uv;

layout (location = 0) out vec4 color;

layout (std140) uniform DirectionalLight
{
    BaseLight base;
    vec3 direction;
    float shadowEnabled;
    mat4 shadowMVP;
} directionalLight;

void main()
{
    float depth = texture(depthMap, vec3(uv,0)).r;
   	vec4 c = texture(gbuffer, vec3(uv, 0));
    vec3 surface_color = c.rgb;
    bool is_far_away = depth > 0.99999;

    vec3 light = vec3(0.0);
    if(!is_far_away)
    {
        vec3 position = WorldPosFromDepth(depth, uv);
        vec4 n = texture(gbuffer, vec3(uv, 1));
        vec3 normal = normalize(n.xyz*2.0 - 1.0);
        float diffuse_intensity = c.w;
        int t = int(floor(n.w*255.0));
        float specular_intensity = float(t & 15) / 15.0;
        float specular_power = 2.0 * float((t & 240) >> 4);

        light = calculate_light(directionalLight.base, directionalLight.direction, position, normal, diffuse_intensity, specular_intensity, specular_power);
        if(directionalLight.shadowEnabled > 0.5) {
            light *= calculate_shadow(directionalLight.shadowMVP, position);
        }
    }
    color = vec4(surface_color * light, 1.0);
}
