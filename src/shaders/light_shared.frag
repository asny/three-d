uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;

uniform mat4 viewProjectionInverse;
uniform vec3 eyePosition;

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

vec3 calculate_light(BaseLight light, vec3 lightDirection, vec3 position, vec3 normal, float diffuse_intensity, float specular_intensity, float specular_power)
{
    float DiffuseFactor = dot(normal, -lightDirection);

    vec3 DiffuseColor  = vec3(0.0);
    vec3 SpecularColor = vec3(0.0);

    if (DiffuseFactor > 0.0)
    {
        DiffuseColor = light.color * diffuse_intensity * light.intensity * DiffuseFactor;

        vec3 VertexToEye = normalize(eyePosition - position);
        vec3 lightReflect = normalize(reflect(lightDirection, normal));
        float SpecularFactor = dot(VertexToEye, lightReflect);
        if (SpecularFactor > 0.0)
        {
            SpecularFactor = pow(SpecularFactor, specular_power);
            SpecularColor = light.color * specular_intensity * light.intensity  * SpecularFactor;
        }
    }

    return DiffuseColor + SpecularColor;
}

vec3 calculate_attenuated_light(BaseLight light, Attenuation attenuation, vec3 light_position, vec3 position, vec3 normal, float diffuse_intensity, float specular_intensity, float specular_power)
{
    vec3 light_direction = position - light_position;
    float distance = length(light_direction);
    light_direction = light_direction / distance;

    vec3 color = calculate_light(light, light_direction, position, normal, diffuse_intensity, specular_intensity, specular_power);

    float att =  attenuation.constant +
        attenuation.linear * distance +
        attenuation.exp * distance * distance;

    return color / max(1.0, att);
}

vec3 WorldPosFromDepth(float depth, vec2 uv) {
    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth * 2.0 - 1.0, 1.0);
    vec4 position = viewProjectionInverse * clipSpacePosition;
    return position.xyz / position.w;
}
