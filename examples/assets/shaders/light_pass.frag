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

struct AmbientLight
{
    BaseLight base;
};

struct DirectionalLight
{
    BaseLight base;
    vec3 direction;
};

struct Attenuation
{
    float constant;
    float linear;
    float exp;
};

struct PointLight
{
    BaseLight base;
    vec3 position;
    Attenuation attenuation;
};

struct SpotLight
{
    PointLight base;
    vec3 direction;
    float cutoff;
};

uniform AmbientLight ambientLight;
uniform DirectionalLight directionalLight;
uniform PointLight pointLight;
uniform SpotLight spotLight;
uniform int lightType;

float is_visible(vec4 shadow_coord, vec2 offset)
{
    return texture(shadowMap, (shadow_coord.xy + offset)/shadow_coord.w).x < (shadow_coord.z - 0.005)/shadow_coord.w ? 0.0f : 1.0f;
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
    return visibility/4.f;
}

vec4 calculate_light(BaseLight light,
                       vec3 lightDirection,
                       vec3 position,
                       vec3 normal)
{
    vec4 surface_parameters = texture(surfaceParametersMap, uv);
    float surface_diffuse_intensity = surface_parameters.x;
    float surface_specular_intensity = surface_parameters.y;
    float surface_specular_power = surface_parameters.z;

    float DiffuseFactor = dot(normal, -lightDirection);

    vec4 DiffuseColor  = vec4(0, 0, 0, 0);
    vec4 SpecularColor = vec4(0, 0, 0, 0);

    if (DiffuseFactor > 0.0)
    {
        DiffuseColor = vec4(light.color * surface_diffuse_intensity * light.intensity * DiffuseFactor, 1.0);

        vec3 VertexToEye = normalize(eyePosition - position);
        vec3 lightReflect = normalize(reflect(lightDirection, normal));
        float SpecularFactor = dot(VertexToEye, lightReflect);
        if (SpecularFactor > 0.0)
        {
            SpecularFactor = pow(SpecularFactor, surface_specular_power);
            SpecularColor = vec4(light.color * surface_specular_intensity * light.intensity  * SpecularFactor, 1.0);
        }
    }

    return DiffuseColor + SpecularColor;
}

vec4 calculate_point_light(vec3 position, vec3 normal)
{
    vec3 lightDirection = position - pointLight.position;
    float distance = length(lightDirection);
    lightDirection = normalize(lightDirection);

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

    vec4 color = shadow * calculate_light(pointLight.base, lightDirection, position, normal);

    float attenuation =  pointLight.attenuation.constant +
        pointLight.attenuation.linear * distance +
        pointLight.attenuation.exp * distance * distance;

    attenuation = max(1.0, attenuation);

    return color / attenuation;
}

void main()
{
    float depth = texture(depthMap, uv).r;
   	vec4 col = vec4(texture(colorMap, uv).xyz, 1.);
    if(depth < 0.99999)
    {
        vec3 position = texture(positionMap, uv).xyz;
        vec3 normal = normalize(texture(normalMap, uv).xyz);

        vec4 light;
        if(lightType == 0)
        {
            light = vec4(ambientLight.base.color * ambientLight.base.intensity, 1.0);
        }
        else if(lightType == 1)
        {
            light = calculate_shadow(position) * calculate_light(directionalLight.base, directionalLight.direction, position, normal);
        }
        else if(lightType == 2)
        {
            light = vec4(1.0);//calculate_point_light(position, normal);
        }

        col *= light;
    }
    color = col;
}
