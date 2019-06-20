uniform sampler2D positionMap;
uniform sampler2D colorMap;
uniform sampler2D normalMap;
uniform sampler2D depthMap;
uniform sampler2D surfaceParametersMap;
uniform samplerCube shadowCubeMap;

layout (location = 0) out vec4 color;

uniform vec3 eyePosition;
uniform mat4 shadowMVP0;
uniform mat4 shadowMVP1;
uniform mat4 shadowMVP2;
uniform mat4 shadowMVP3;
uniform mat4 shadowMVP4;
uniform mat4 shadowMVP5;

in vec2 uv;

const int MAX_NO_LIGHTS = 4;
uniform sampler2DArray directionalLightShadowMaps;
uniform sampler2DArray spotLightShadowMaps;

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

struct AmbientLight
{
    BaseLight base;
};

struct DirectionalLight
{
    BaseLight base;
    vec3 direction;
    float padding;
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
    float padding;
    mat4 shadowMVP;
};

uniform AmbientLight ambientLight;

layout (std140) uniform DirectionalLights
{
    DirectionalLight directionalLights[MAX_NO_LIGHTS];
};

layout (std140) uniform PointLights
{
    PointLight pointLights[MAX_NO_LIGHTS];
};

layout (std140) uniform SpotLights
{
    SpotLight spotLights[MAX_NO_LIGHTS];
};

float is_visible(int lightIndex, sampler2DArray shadowMap, vec4 shadow_coord, vec2 offset)
{
    vec2 uv = (shadow_coord.xy + offset)/shadow_coord.w;
    float true_distance = (shadow_coord.z - 0.005)/shadow_coord.w;
    float shadow_cast_distance = texture(shadowMap, vec3(uv, lightIndex)).x;
    return uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0 || shadow_cast_distance > true_distance ? 1.0 : 0.0;
}

float calculate_shadow(int lightIndex, sampler2DArray shadowMap, mat4 shadowMVP, vec3 position)
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
        visibility += is_visible(lightIndex, shadowMap, shadow_coord, poissonDisk[i] * 0.001f);
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

vec3 calculate_spot_light(int i, vec3 position)
{
    SpotLight spotLight = spotLights[i];
    if(spotLight.base.intensity > 0.0)
    {
        vec3 light_direction = normalize(position - spotLight.position);
        float SpotFactor = dot(light_direction, spotLight.direction);

        if (SpotFactor > spotLight.cutoff) {
            return calculate_shadow(i, spotLightShadowMaps, spotLight.shadowMVP, position) *
                calculate_attenuated_light(spotLight.base, spotLight.attenuation, spotLight.position, position)
                * (1.0 - (1.0 - SpotFactor) * 1.0/(1.0 - spotLight.cutoff));
        }
    }
    return vec3(0.0);
}

void main()
{
    float depth = texture(depthMap, uv).r;
   	vec3 surface_color = texture(colorMap, uv).rgb;
    bool is_far_away = depth > 0.99999;
    vec3 position = texture(positionMap, uv).xyz;

    vec3 light = ambientLight.base.color * (is_far_away? 1.0 : ambientLight.base.intensity);
    if(!is_far_away)
    {
        for(int i = 0; i < MAX_NO_LIGHTS; i++)
        {
            DirectionalLight directionalLight = directionalLights[i];
            if(directionalLight.base.intensity > 0.0)
            {
                light += calculate_shadow(i, directionalLightShadowMaps, directionalLight.shadowMVP, position)
                    * calculate_light(directionalLight.base, directionalLight.direction, position);
            }

            PointLight pointLight = pointLights[i];
            if(pointLight.base.intensity > 0.0)
            {
                light += calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, position);
            }

            light += calculate_spot_light(i, position);

        }
    }

    color = vec4(surface_color * light, 1.0);
}
