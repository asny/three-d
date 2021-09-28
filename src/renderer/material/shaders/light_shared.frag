
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

#define PI 3.1415926

// handy value clamping to 0 - 1 range
float saturate(in float value)
{
    return clamp(value, 0.0, 1.0);
}

// compute fresnel specular factor
// cosTheta could be NdV or VdH depending on used technique
vec3 fresnel_factor(in vec3 F0, in float cosTheta)
{
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}


// following functions are copies of UE4
// for computing cook-torrance specular lighting terms

float D_blinn(in float roughness, in float NdH)
{
    float alpha = roughness * roughness;
    float alpha2 = alpha * alpha;
    float n = 2.0 / alpha2 - 2.0;
    return (n + 2.0) / (2.0 * PI) * pow(NdH, n);
}

float D_beckmann(in float roughness, in float NdH)
{
    float alpha = roughness * roughness;
    float alpha2 = alpha * alpha;
    float NdH2 = NdH * NdH;
    return exp((NdH2 - 1.0) / (alpha2 * NdH2)) / (PI * alpha2 * NdH2 * NdH2);
}

// Trowbridge-Reitz GGX normal distribution function
float D_GGX(in float roughness, in float NdH)
{
    float alpha = roughness * roughness;
    float alpha2 = alpha * alpha;
    float d = (NdH * alpha2 - NdH) * NdH + 1.0;
    return alpha2 / (PI * d * d);
}

// Smith's Schlick-GGX geometry function including the normalization factor 1 / (4 * NdV * NdL)
float G_schlick(in float roughness, in float NdV, in float NdL)
{
    float alpha = roughness * roughness;
    float k = 0.125 * (alpha + 1.0) * (alpha + 1.0);
    float V = NdV * (1.0 - k) + k;
    float L = NdL * (1.0 - k) + k;
    return 0.25 / (V * L);
}


// simple phong specular calculation with normalization
vec3 phong_specular(in vec3 V, in vec3 L, in vec3 N, in vec3 specular_fresnel, in float roughness)
{
    vec3 R = reflect(-L, N);
    float VdR = max(0.0, dot(V, R));

    float k = 1.999 / (roughness * roughness);

    return min(1.0, 3.0 * 0.0398 * k) * pow(VdR, min(10000.0, k)) * specular_fresnel;
}

// simple blinn specular calculation with normalization
vec3 blinn_specular(in float NdH, in vec3 specular_fresnel, in float roughness)
{
    float k = 1.999 / (roughness * roughness);
    
    return min(1.0, 3.0 * 0.0398 * k) * pow(NdH, min(10000.0, k)) * specular_fresnel;
}

// cook-torrance specular calculation                      
vec3 cooktorrance_specular(in float NdL, in float NdV, in float NdH, in vec3 specular_fresnel, in float roughness)
{
    float D = 0.0;
#ifdef COOK_BLINN
    D = D_blinn(roughness, NdH);
#endif

#ifdef COOK_BECKMANN
    D = D_beckmann(roughness, NdH);
#endif

#ifdef COOK_GGX
    D = D_GGX(roughness, NdH);
#endif

    float G = G_schlick(roughness, NdV, NdL);

    return specular_fresnel * G * D;
}

vec3 calculate_light(vec3 light_color, vec3 L, vec3 surface_color, vec3 position, vec3 N, float metallic, float roughness, float occlusion)
{
    vec3 V = normalize(eyePosition - position);

    // compute material reflectance
    float NdL = max(0.0, dot(N, L));
    float NdV = max(0.001, dot(N, V));

    // mix between metal and non-metal material, for non-metal
    // constant base specular factor of 0.04 grey is used
    vec3 specular = mix(vec3(0.04), surface_color, metallic);

#ifdef PHONG
    // specular reflectance with PHONG
    vec3 specular_fresnel = fresnel_factor(specular, NdV);
    vec3 reflected_light = phong_specular(V, L, N, specular_fresnel, roughness);
#else
    vec3 H = normalize(L + V);
    float NdH = max(0.001, dot(N, H));
    float HdV = max(0.001, dot(H, V));
#endif

#ifdef BLINN
    // specular reflectance with BLINN
    vec3 specular_fresnel = fresnel_factor(specular, HdV);
    vec3 reflected_light = blinn_specular(NdH, specular_fresnel, roughness);
#endif

#ifdef COOK
    // specular reflectance with COOK-TORRANCE
    vec3 specular_fresnel = fresnel_factor(specular, HdV);
    vec3 reflected_light = cooktorrance_specular(NdL, NdV, NdH, specular_fresnel, roughness);
#endif

    // diffuse is common for any model
    vec3 diffuse_fresnel = 1.0 - specular_fresnel;
    vec3 diffuse_light = diffuse_fresnel * mix(surface_color, vec3(0.0), metallic) / PI;
    
    // final result
    return (occlusion * diffuse_light + reflected_light) * light_color * NdL;
}

vec3 calculate_attenuated_light(vec3 light_color, Attenuation attenuation, vec3 light_position, vec3 surface_color, vec3 position, vec3 normal, float metallic, float roughness, float occlusion)
{
    vec3 light_direction = light_position - position;
    float distance = length(light_direction);
    light_direction = light_direction / distance;

    float att =  attenuation.constant +
        attenuation.linear * distance +
        attenuation.exp * distance * distance;

    return calculate_light(light_color / max(1.0, att), light_direction, surface_color, position, normal, metallic, roughness, occlusion);
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
    float metallic, float roughness, float occlusion, sampler2D shadowMap)
{
    vec3 light_color = directionalLight.base.intensity * directionalLight.base.color;
    vec3 light = calculate_light(light_color, -directionalLight.direction, surface_color, position, normal, metallic, roughness, occlusion);
    if(directionalLight.shadowEnabled > 0.5) {
        light *= calculate_shadow(shadowMap, directionalLight.shadowMVP, position);
    }
    return light;
}

vec3 calculate_point_light(PointLight pointLight, vec3 surface_color, vec3 position, vec3 normal,
    float metallic, float roughness, float occlusion)
{
    vec3 light_color = pointLight.base.intensity * pointLight.base.color;
    return calculate_attenuated_light(light_color, pointLight.attenuation, pointLight.position, surface_color, position, normal,
        metallic, roughness, occlusion);
}

vec3 calculate_spot_light(SpotLight spotLight, vec3 surface_color, vec3 position, vec3 normal,
    float metallic, float roughness, float occlusion, sampler2D shadowMap)
{
    vec3 light_color = spotLight.base.intensity * spotLight.base.color;
    vec3 light_direction = normalize(position - spotLight.position);
    float angle = acos(dot(light_direction, normalize(spotLight.direction)));
    float cutoff = 3.14 * spotLight.cutoff / 180.0;

    vec3 light = vec3(0.0);
    if (angle < cutoff) {
        light = calculate_attenuated_light(light_color, spotLight.attenuation, spotLight.position, surface_color, position, normal, 
            metallic, roughness, occlusion) * (1.0 - smoothstep(0.75 * cutoff, cutoff, angle));
        if(spotLight.shadowEnabled > 0.5) {
            light *= calculate_shadow(shadowMap, spotLight.shadowMVP, position);
        }
    }
    return light;
}