
layout (location = 0) out vec4 color;

layout (std140) uniform SpotLight
{
    BaseLight base;
    Attenuation attenuation;
    vec3 position;
    float cutoff;
    vec3 direction;
    float shadowEnabled;
    mat4 shadowMVP;
} spotLight;

void main()
{
    Surface surface = get_surface();
    vec3 light_direction = normalize(surface.position - spotLight.position);
    float angle = acos(dot(light_direction, normalize(spotLight.direction)));
    float cutoff = 3.14 * spotLight.cutoff / 180.0;

    vec3 light = vec3(0.0);
    if (angle < cutoff) {
        light = calculate_attenuated_light(spotLight.base, spotLight.attenuation, spotLight.position, surface) * (1.0 - smoothstep(0.75 * cutoff, cutoff, angle));
        if(spotLight.shadowEnabled > 0.5) {
            light *= calculate_shadow(spotLight.shadowMVP, surface.position);
        }
    }
    color = vec4(surface.color * light, 1.0);
}
