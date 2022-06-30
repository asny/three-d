
struct BaseLight
{
    vec3 color;
    float intensity;
};

// compute fresnel specular factor
// cosTheta could be NdV or VdH depending on used technique
vec3 fresnel_schlick(vec3 F0, float cosTheta)
{
    return F0 + (1.0 - F0) * pow(saturate(1.0 - cosTheta), 5.0);
}

vec3 fresnel_schlick_roughness(vec3 F0, float cosTheta, float roughness)
{
    return F0 + (max(vec3(1.0 - roughness), F0) - F0) * pow(saturate(1.0 - cosTheta), 5.0);
} 


// following functions are copies of UE4
// for computing cook-torrance specular lighting terms

float D_blinn(in float roughness, in float NdH)
{
    float alpha = roughness * roughness;
    float alpha2 = max(alpha * alpha, 0.001);
    float n = 2.0 / alpha2 - 2.0;
    return (n + 2.0) / (2.0 * PI) * pow(NdH, n);
}

float D_beckmann(in float roughness, in float NdH)
{
    float alpha = roughness * roughness;
    float alpha2 = max(alpha * alpha, 0.001);
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

float calculate_D(float roughness, float NdH) {
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
    return D;
}

// Smith's Schlick-GGX geometry function
float G_schlick(in float roughness, in float NdV, in float NdL)
{
    float alpha = roughness * roughness;
    float k = 0.125 * (alpha + 1.0) * (alpha + 1.0);
    float V = NdV * (1.0 - k) + k;
    float L = NdL * (1.0 - k) + k;
    return NdV * NdL / (V * L);
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
    float D = calculate_D(roughness, NdH);
    float G = G_schlick(roughness, NdV, NdL);
    return specular_fresnel * G * D / (4.0 * NdV * NdL);
}

vec3 calculate_light(vec3 light_color, vec3 L, vec3 surface_color, vec3 V, vec3 N, float metallic, float roughness)
{
    // compute material reflectance
    float NdL = max(0.001, dot(N, L));
    float NdV = max(0.001, dot(N, V));

    // mix between metal and non-metal material, for non-metal
    // constant base specular factor of 0.04 grey is used
    vec3 F0 = mix(vec3(0.04), surface_color, metallic);

#ifdef PHONG
    // specular reflectance with PHONG
    vec3 specular_fresnel = fresnel_schlick_roughness(F0, NdV, roughness);
    vec3 specular = phong_specular(V, L, N, specular_fresnel, roughness);
#else
    vec3 H = normalize(L + V);
    float NdH = max(0.001, dot(N, H));
    float HdV = max(0.001, dot(H, V));
    vec3 specular_fresnel = fresnel_schlick_roughness(F0, HdV, roughness);
#endif

#ifdef BLINN
    // specular reflectance with BLINN
    vec3 specular = blinn_specular(NdH, specular_fresnel, roughness);
#endif

#ifdef COOK
    // specular reflectance with COOK-TORRANCE
    vec3 specular = cooktorrance_specular(NdL, NdV, NdH, specular_fresnel, roughness);
#endif

    // diffuse is common for any model
    vec3 diffuse_fresnel = 1.0 - specular_fresnel;
    vec3 diffuse = diffuse_fresnel * mix(surface_color, vec3(0.0), metallic) / PI;
    
    // final result
    return (diffuse + specular) * light_color * NdL;
}

vec3 attenuate(vec3 light_color, vec3 attenuation, float distance)
{
    float att =  attenuation.x +
        attenuation.y * distance +
        attenuation.z * distance * distance;

    return light_color / max(1.0, att);
}

float is_visible(sampler2D shadowMap, vec4 shadow_coord, vec2 offset)
{
    vec2 uv = (shadow_coord.xy + offset)/shadow_coord.w;
    if(uv.x < 0.0 || uv.x > 1.0 || uv.y < 0.0 || uv.y > 1.0) {
        return 1.0;
    }
    float shadow_cast_distance = texture(shadowMap, uv).x;
    if(shadow_cast_distance > 0.999) {
        return 1.0;
    }
    float true_distance = (shadow_coord.z - 0.005)/shadow_coord.w;
    return shadow_cast_distance > true_distance ? 1.0 : 0.0;
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

vec3 ImportanceSampleGGX(vec2 Xi, vec3 N, float roughness)
{
	float a = roughness*roughness;
	
	float phi = 2.0 * PI * Xi.x;
	float cosTheta = sqrt((1.0 - Xi.y) / (1.0 + (a*a - 1.0) * Xi.y));
	float sinTheta = sqrt(1.0 - cosTheta*cosTheta);
	
	// from spherical coordinates to cartesian coordinates - halfway vector
	vec3 H;
	H.x = cos(phi) * sinTheta;
	H.y = sin(phi) * sinTheta;
	H.z = cosTheta;
	
	// from tangent-space H vector to world-space sample vector
	vec3 up          = abs(N.z) < 0.999 ? vec3(0.0, 0.0, 1.0) : vec3(1.0, 0.0, 0.0);
	vec3 tangent   = normalize(cross(up, N));
	vec3 bitangent = cross(N, tangent);
	
	vec3 sampleVec = tangent * H.x + bitangent * H.y + N * H.z;
	return normalize(sampleVec);
}
