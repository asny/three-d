
uniform float metallic;
uniform float roughness;
uniform vec3 cameraPosition;

uniform vec4 albedo;
#ifdef USE_ALBEDO_TEXTURE
uniform sampler2D albedoTexture;
#endif

uniform vec4 emissive;
#ifdef USE_EMISSIVE_TEXTURE
uniform sampler2D emissiveTexture;
#endif

#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
uniform sampler2D metallicRoughnessTexture;
#endif

#ifdef USE_OCCLUSION_TEXTURE
uniform sampler2D occlusionTexture;
uniform float occlusionStrength;
#endif

#ifdef USE_NORMAL_TEXTURE
uniform sampler2D normalTexture;
uniform float normalScale;
#endif

in vec3 pos;
in vec3 nor;

layout (location = 0) out vec4 outColor;

void main()
{
    vec4 surface_color = albedo;
#ifdef USE_ALBEDO_TEXTURE
    vec4 c = texture(albedoTexture, uvs);
    #ifdef ALPHACUT
        if (c.a < acut) discard;
    #endif
    surface_color *= vec4(rgb_from_srgb(c.rgb), c.a);
#endif
#ifdef USE_VERTEX_COLORS
    surface_color *= col;
#endif

    float metallic_factor = metallic;
    float roughness_factor = roughness;
#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
    vec2 t = texture(metallicRoughnessTexture, uvs).gb;
    roughness_factor *= t.x;
    metallic_factor *= t.y;
#endif

    float occlusion = 1.0;
#ifdef USE_OCCLUSION_TEXTURE
    occlusion = mix(1.0, texture(occlusionTexture, uvs).r, occlusionStrength);
#endif

    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
#ifdef USE_NORMAL_TEXTURE
    vec3 tangent = normalize(gl_FrontFacing ? tang : -tang);
    vec3 bitangent = normalize(gl_FrontFacing ? bitang : -bitang);
    mat3 tbn = mat3(tangent, bitangent, normal);
    normal = tbn * ((2.0 * texture(normalTexture, uvs).xyz - 1.0) * vec3(normalScale, normalScale, 1.0));
#endif

    vec3 total_emissive = emissive.rgb;
#ifdef USE_EMISSIVE_TEXTURE
    vec4 e = texture(emissiveTexture, uvs);
    total_emissive *= rgb_from_srgb(e.rgb);
#endif

    outColor.rgb = total_emissive + calculate_lighting(cameraPosition, surface_color.rgb, pos, normal, metallic_factor, roughness_factor, occlusion);
    outColor.rgb = reinhard_tone_mapping(outColor.rgb);
    outColor.rgb = srgb_from_rgb(outColor.rgb);
    outColor.a = surface_color.a;
}