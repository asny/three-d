
uniform vec3 eyePosition;

struct Surface
{
    vec3 position;
    vec3 normal;
    vec4 color;
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

struct DirectionalLight
{
    BaseLight base;
    vec3 direction;
    float shadowEnabled;
    mat4 shadowMVP;
};

struct PointLight
{
    BaseLight base;
    Attenuation attenuation;
    vec3 position;
    float padding;
};

struct SpotLight
{
    BaseLight base;
    Attenuation attenuation;
    vec3 position;
    float cutoff;
    vec3 direction;
    float shadowEnabled;
    mat4 shadowMVP;
};

vec3 calculate_light(BaseLight light, vec3 lightDirection, vec3 position, vec3 normal,
    float diffuse_intensity, float specular_intensity, float specular_power)
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

vec3 calculate_attenuated_light(BaseLight light, Attenuation attenuation, vec3 light_position, vec3 position, vec3 normal,
    float diffuse_intensity, float specular_intensity, float specular_power)
{
    vec3 light_direction = position - light_position;
    float distance = length(light_direction);
    light_direction = light_direction / distance;

    vec3 color = calculate_light(light, light_direction, position, normal,
        diffuse_intensity, specular_intensity, specular_power);

    float att =  attenuation.constant +
        attenuation.linear * distance +
        attenuation.exp * distance * distance;

    return color / max(1.0, att);
}

float is_visible(sampler2D shadowMap, vec4 shadow_coord, vec2 offset)
{
    vec2 uv = (shadow_coord.xy + offset)/shadow_coord.w;
    float true_distance = (shadow_coord.z - 0.005)/shadow_coord.w;
    float shadow_cast_distance = texture(shadowMap, uv).x;
    return uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || shadow_cast_distance > true_distance ? 1.0 : 0.0;
}

float calculate_shadow(sampler2D shadowMap, mat4 shadowMVP, vec3 position)
{
    if(shadowMVP[3][3] < 0.1) // Shadow disabled
    {
        return 1.0;
    }
    vec4 shadow_coord = shadowMVP * vec4(position, 1.);
    float visibility = 0.0;
    vec2 poissonDisk[4] = vec2[](
                                 vec2( -0.94201624, -0.39906216 ),
                                 vec2( 0.94558609, -0.76890725 ),
                                 vec2( -0.094184101, -0.92938870 ),
                                 vec2( 0.34495938, 0.29387760 )
                                 );
    for (int i=0;i<4;i++)
    {
        visibility += is_visible(shadowMap, shadow_coord, poissonDisk[i] * 0.001f);
    }
    return visibility * 0.25;
}

vec3 calculate_directional_light(DirectionalLight directionalLight, vec3 surface_color, vec3 position, vec3 normal,
    float diffuse_intensity, float specular_intensity, float specular_power, sampler2D shadowMap)
{
    vec3 light = calculate_light(directionalLight.base, directionalLight.direction, position, normal,
        diffuse_intensity, specular_intensity, specular_power);
    if(directionalLight.shadowEnabled > 0.5) {
        light *= calculate_shadow(shadowMap, directionalLight.shadowMVP, position);
    }
    return surface_color * light;
}

vec3 calculate_point_light(PointLight pointLight, vec3 surface_color, vec3 position, vec3 normal,
    float diffuse_intensity, float specular_intensity, float specular_power)
{
    return surface_color * calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, position, normal,
        diffuse_intensity, specular_intensity, specular_power);
}

vec3 calculate_spot_light(SpotLight spotLight, vec3 surface_color, vec3 position, vec3 normal,
    float diffuse_intensity, float specular_intensity, float specular_power, sampler2D shadowMap)
{
    vec3 light_direction = normalize(position - spotLight.position);
    float angle = acos(dot(light_direction, normalize(spotLight.direction)));
    float cutoff = 3.14 * spotLight.cutoff / 180.0;

    vec3 light = vec3(0.0);
    if (angle < cutoff) {
        light = calculate_attenuated_light(spotLight.base, spotLight.attenuation, spotLight.position, position, normal,
            diffuse_intensity, specular_intensity, specular_power) * (1.0 - smoothstep(0.75 * cutoff, cutoff, angle));
        if(spotLight.shadowEnabled > 0.5) {
            light *= calculate_shadow(shadowMap, spotLight.shadowMVP, position);
        }
    }
    return surface_color * light;
}