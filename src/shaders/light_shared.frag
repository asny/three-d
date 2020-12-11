
uniform vec3 eyePosition;

struct Surface
{
    vec3 position;
    vec3 normal;
    vec3 color;
    float diffuse_intensity;
    float specular_intensity;
    float specular_power;
};

struct BaseLight
{
    vec3 color;
    float intensity;
};

struct Attenuation
{
    float constant;
    float linear;
    float exp;
    float padding;
};

vec3 calculate_light(BaseLight light, vec3 lightDirection, Surface surface)
{
    float DiffuseFactor = dot(surface.normal, -lightDirection);

    vec3 DiffuseColor  = vec3(0.0);
    vec3 SpecularColor = vec3(0.0);

    if (DiffuseFactor > 0.0)
    {
        DiffuseColor = light.color * surface.diffuse_intensity * light.intensity * DiffuseFactor;

        vec3 VertexToEye = normalize(eyePosition - surface.position);
        vec3 lightReflect = normalize(reflect(lightDirection, surface.normal));
        float SpecularFactor = dot(VertexToEye, lightReflect);
        if (SpecularFactor > 0.0)
        {
            SpecularFactor = pow(SpecularFactor, surface.specular_power);
            SpecularColor = light.color * surface.specular_intensity * light.intensity  * SpecularFactor;
        }
    }

    return DiffuseColor + SpecularColor;
}

vec3 calculate_attenuated_light(BaseLight light, Attenuation attenuation, vec3 light_position, Surface surface)
{
    vec3 light_direction = surface.position - light_position;
    float distance = length(light_direction);
    light_direction = light_direction / distance;

    vec3 color = calculate_light(light, light_direction, surface);

    float att =  attenuation.constant +
        attenuation.linear * distance +
        attenuation.exp * distance * distance;

    return color / max(1.0, att);
}
