
uniform vec3 eyePosition;
uniform sampler2D shadowMap;

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

float is_visible(vec4 shadow_coord, vec2 offset)
{
    vec2 uv = (shadow_coord.xy + offset)/shadow_coord.w;
    float true_distance = (shadow_coord.z - 0.005)/shadow_coord.w;
    float shadow_cast_distance = texture(shadowMap, uv).x;
    return uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || shadow_cast_distance > true_distance ? 1.0 : 0.0;
}

float calculate_shadow(mat4 shadowMVP, vec3 position)
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
        visibility += is_visible(shadow_coord, poissonDisk[i] * 0.001f);
    }
    return visibility * 0.25;
}

vec3 calculate_ambient_light(BaseLight ambientLight, Surface surface)
{
    return surface.color * ambientLight.color * ambientLight.intensity;
}

vec3 calculate_directional_light(DirectionalLight directionalLight, Surface surface)
{
    vec3 light = calculate_light(directionalLight.base, directionalLight.direction, surface);
    if(directionalLight.shadowEnabled > 0.5) {
        light *= calculate_shadow(directionalLight.shadowMVP, surface.position);
    }
    return surface.color * light;
}

vec3 calculate_point_light(PointLight pointLight, Surface surface)
{
    return surface.color * calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, surface);
}

vec3 calculate_spot_light(SpotLight spotLight, Surface surface)
{
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
    return surface.color * light;
}