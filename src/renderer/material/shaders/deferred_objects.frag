
uniform float metallic;
uniform float roughness;

uniform vec4 albedo;
#ifdef USE_ALBEDO_TEXTURE
uniform sampler2D albedoTexture;
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

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void main()
{
    vec4 surface_color = albedo;
#ifdef USE_ALBEDO_TEXTURE
    vec4 c = texture(albedoTexture, uvs);
    surface_color *= vec4(rgb_from_srgb(c.rgb), c.a);
#endif

    float metallic_factor = metallic;
    float roughness_factor = roughness;
#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
    vec2 t = texture(metallicRoughnessTexture, uvs).xy;
    metallic_factor *= t.x;
    roughness_factor *= t.y;
#endif

    float occlusion = 1.0;
#ifdef USE_OCCLUSION_TEXTURE
    occlusion = mix(1.0, texture(occlusionTexture, uvs).r, occlusionStrength);
#endif

    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
#ifdef USE_NORMAL_TEXTURE
    mat3 tbn = basis(normal);
    normal = tbn * ((2.0 * texture(normalTexture, uvs).xyz - 1.0) * vec3(normalScale, normalScale, 1.0));
#endif

    out_color = vec4(surface_color.rgb, metallic_factor);
    out_normal = vec4(0.5 * normal.xy + 0.5, occlusion, roughness_factor);
}