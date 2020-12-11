

layout (location = 0) out vec4 color;

layout (std140) uniform DirectionalLight
{
    BaseLight base;
    vec3 direction;
    float shadowEnabled;
    mat4 shadowMVP;
} directionalLight;

vec3 calculate_directional_light(Surface surface)
{
    vec3 light = calculate_light(directionalLight.base, directionalLight.direction, surface.position, surface.normal,
        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power);
    if(directionalLight.shadowEnabled > 0.5) {
        light *= calculate_shadow(directionalLight.shadowMVP, surface.position);
    }
    return surface.color * light;
}

void main()
{
    Surface surface = get_surface();
    color = vec4(calculate_directional_light(surface), 1.0);
}
