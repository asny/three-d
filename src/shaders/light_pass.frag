uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;
uniform samplerCube shadowCubeMap;

layout (location = 0) out vec4 color;

uniform mat4 viewProjectionInverse;
uniform int light_type;

uniform vec3 eyePosition;
uniform mat4 shadowMVP0;
uniform mat4 shadowMVP1;
uniform mat4 shadowMVP2;
uniform mat4 shadowMVP3;
uniform mat4 shadowMVP4;
uniform mat4 shadowMVP5;

in vec2 uv;

const int MAX_NO_LIGHTS = 4;
uniform sampler2D shadowMap;

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

uniform AmbientLight ambientLight;

layout (std140) uniform DirectionalLight
{
    BaseLight base;
    vec3 direction;
    float padding;
    mat4 shadowMVP;
} directionalLight;

layout (std140) uniform PointLight
{
    BaseLight base;
    Attenuation attenuation;
    vec3 position;
    float padding;
} pointLight;

layout (std140) uniform SpotLight
{
    BaseLight base;
    Attenuation attenuation;
    vec3 position;
    float cutoff;
    vec3 direction;
    float padding;
    mat4 shadowMVP;
} spotLight;

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

vec3 WorldPosFromDepth(float depth, vec2 uv) {
    vec4 clipSpacePosition = vec4(uv * 2.0 - 1.0, depth * 2.0 - 1.0, 1.0);
    vec4 position = viewProjectionInverse * clipSpacePosition;
    return position.xyz / position.w;
}

void main()
{
    float depth = texture(depthMap, vec3(uv,0)).r;
   	vec4 c = texture(gbuffer, vec3(uv, 0));
    vec3 surface_color = c.rgb;
    bool is_far_away = depth > 0.99999;

    color = vec4(0.0, 0.0, 0.0, 0.0);
    if(light_type == 0) { // Ambient light
        color = vec4(surface_color * ambientLight.base.color * (is_far_away? 1.0 : ambientLight.base.intensity), 0.0);
    }
    else if(!is_far_away)
    {
        vec3 position = WorldPosFromDepth(depth, uv);
        vec4 n = texture(gbuffer, vec3(uv, 1));
        vec3 normal = normalize(n.xyz*2.0 - 1.0);
        float diffuse_intensity = c.w;
        int t = int(floor(n.w*255.0));
        float specular_intensity = float(t & 15) / 15.0;
        float specular_power = 2.0 * float((t & 240) >> 4);

        vec3 light = vec3(0.0);
        if(light_type == 1 || light_type == 2) // Directional light
        {
            light = calculate_light(directionalLight.base, directionalLight.direction, position, normal, diffuse_intensity, specular_intensity, specular_power);
            if(light_type == 2)
                light *= calculate_shadow(directionalLight.shadowMVP, position);
        }
        else if(light_type == 3 || light_type == 4) // Spot light
        {
            vec3 light_direction = normalize(position - spotLight.position);
            float SpotFactor = dot(light_direction, spotLight.direction);

            if (SpotFactor > spotLight.cutoff) {
                light = calculate_attenuated_light(spotLight.base, spotLight.attenuation, spotLight.position, position, normal, diffuse_intensity, specular_intensity, specular_power)
                    * (1.0 - (1.0 - SpotFactor) * 1.0/(1.0 - spotLight.cutoff));
                if(light_type == 4)
                    light *= calculate_shadow(spotLight.shadowMVP, position);
            }
        }
        else if(light_type == 5) {
            light = calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, position, normal, diffuse_intensity, specular_intensity, specular_power);
        }
        color.rgb = surface_color * light;
    }
}
