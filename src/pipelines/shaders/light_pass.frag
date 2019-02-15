uniform sampler2D positionMap;
uniform sampler2D colorMap;
uniform sampler2D normalMap;
uniform sampler2D depthMap;
uniform sampler2D surfaceParametersMap;
uniform sampler2D shadowMap;
uniform samplerCube shadowCubeMap;

layout (location = 0) out vec4 color;

uniform vec3 eyePosition;
uniform mat4 shadowMVP;
uniform mat4 shadowMVP0;
uniform mat4 shadowMVP1;
uniform mat4 shadowMVP2;
uniform mat4 shadowMVP3;
uniform mat4 shadowMVP4;
uniform mat4 shadowMVP5;

in vec2 uv;

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
};

struct AmbientLight
{
    BaseLight base;
};

struct DirectionalLight
{
    BaseLight base;
    vec3 direction;
};

struct PointLight
{
    BaseLight base;
    vec3 position;
    Attenuation attenuation;
};

struct SpotLight
{
    BaseLight base;
    vec3 position;
    vec3 direction;
    Attenuation attenuation;
    float cutoff;
};

uniform AmbientLight ambientLight;
uniform DirectionalLight directionalLight;
uniform PointLight pointLight;
uniform SpotLight spotLight;
uniform int lightType;

float is_visible(vec4 shadow_coord, vec2 offset)
{
    vec2 uv = (shadow_coord.xy + offset)/shadow_coord.w;
    float true_distance = (shadow_coord.z - 0.005)/shadow_coord.w;
    float shadow_cast_distance = texture(shadowMap, uv).x;
    return uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || shadow_cast_distance > true_distance ? 1.0 : 0.0;
}

float calculate_shadow(vec3 position)
{
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

vec3 calculate_light(BaseLight light, vec3 lightDirection, vec3 position)
{
    vec3 normal = normalize(texture(normalMap, uv).xyz);
    vec4 surface_parameters = texture(surfaceParametersMap, uv);
    float surface_diffuse_intensity = surface_parameters.x;
    float surface_specular_intensity = surface_parameters.y;
    float surface_specular_power = surface_parameters.z;

    float DiffuseFactor = dot(normal, -lightDirection);

    vec3 DiffuseColor  = vec3(0.0);
    vec3 SpecularColor = vec3(0.0);

    if (DiffuseFactor > 0.0)
    {
        DiffuseColor = light.color * surface_diffuse_intensity * light.intensity * DiffuseFactor;

        vec3 VertexToEye = normalize(eyePosition - position);
        vec3 lightReflect = normalize(reflect(lightDirection, normal));
        float SpecularFactor = dot(VertexToEye, lightReflect);
        if (SpecularFactor > 0.0)
        {
            SpecularFactor = pow(SpecularFactor, surface_specular_power);
            SpecularColor = light.color * surface_specular_intensity * light.intensity  * SpecularFactor;
        }
    }

    return DiffuseColor + SpecularColor;
}

vec3 calculate_attenuated_light(BaseLight light, Attenuation attenuation, vec3 light_position, vec3 position)
{
    vec3 light_direction = position - light_position;
    float distance = length(light_direction);
    light_direction = light_direction / distance;

    vec3 color = calculate_light(light, light_direction, position);

    float att =  attenuation.constant +
        attenuation.linear * distance +
        attenuation.exp * distance * distance;

    return color / max(1.0, att);
}

vec3 calculate_directional_light(vec3 position)
{
    return calculate_shadow(position) * calculate_light(directionalLight.base, directionalLight.direction, position);
}

vec3 calculate_point_light(vec3 position)
{
    vec3 color = calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, position);

    /*mat4 shadowMatrix;
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
    }*/

    return color;
}

vec3 calculate_spot_light(vec3 position)
{
    vec3 light_direction = normalize(position - spotLight.position);
    float SpotFactor = dot(light_direction, spotLight.direction);

    if (SpotFactor > spotLight.cutoff) {
        vec3 color = calculate_attenuated_light(spotLight.base, spotLight.attenuation, spotLight.position, position);
        return calculate_shadow(position) * color * (1.0 - (1.0 - SpotFactor) * 1.0/(1.0 - spotLight.cutoff));
    }
    else {
        return vec3(0.0);
    }
}

void main()
{
    float depth = texture(depthMap, uv).r;
   	vec3 surface_color = texture(colorMap, uv).rgb;
    bool is_far_away = depth > 0.99999;
    vec3 position = texture(positionMap, uv).xyz;

    vec3 light = vec3(0.0);
    if(lightType == 0)
    {
        light = ambientLight.base.color * (is_far_away? 1.0 : ambientLight.base.intensity);
    }
    else if(lightType == 1)
    {
        if(!is_far_away)
        {
            light = calculate_directional_light(position);
        }
    }
    else if(lightType == 2)
    {
        if(!is_far_away)
        {
            light = calculate_point_light(position);
        }
    }
    else if(lightType == 3)
    {
        if(!is_far_away)
        {
            light = calculate_spot_light(position);
        }
    }

    color = vec4(surface_color * light, 1.0);
}
