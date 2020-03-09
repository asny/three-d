
in vec2 uv;

layout (location = 0) out vec4 color;

//uniform samplerCube shadowCubeMap;
/*uniform mat4 shadowMVP0;
uniform mat4 shadowMVP1;
uniform mat4 shadowMVP2;
uniform mat4 shadowMVP3;
uniform mat4 shadowMVP4;
uniform mat4 shadowMVP5;*/

layout (std140) uniform PointLight
{
    BaseLight base;
    Attenuation attenuation;
    vec3 position;
    float padding;
} pointLight;

/*vec3 calculate_point_light(vec3 position)
{
    vec3 color = calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, position);

    mat4 shadowMatrix;
    float x = abs(lightDirection.x);
    float y = abs(lightDirection.y);
    float z = abs(lightDirection.z);
    if(x > y && x > z)
    {
        if(lightDirection.x > 0.0)
        {
            shadowMatrix = shadowMVP0;
        }
        else {
            shadowMatrix = shadowMVP1;
        }
    }
    else if(y > x && y > z)
    {
        if(lightDirection.y > 0.0)
        {
            shadowMatrix = shadowMVP2;
        }
        else {
            shadowMatrix = shadowMVP3;
        }
    }
    else if(z > x && z > y)
    {
        if(lightDirection.z > 0.0)
        {
            shadowMatrix = shadowMVP4;
        }
        else {
            shadowMatrix = shadowMVP5;
        }
    }
    else {
        return vec4(1., 0., 0., 1.);
    }

    float shadow = 1.f;
    vec4 shadow_coord = shadowMatrix * vec4(position, 1.);
    if ( texture(shadowCubeMap, lightDirection).x < (shadow_coord.z - 0.005)/shadow_coord.w)
    {
        shadow = 0.5f;
    }

    return color;
}*/

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
        light = calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position,
            position, normal, diffuse_intensity, specular_intensity, specular_power);
    }
    color = vec4(surface_color * light, 1.0);
}
