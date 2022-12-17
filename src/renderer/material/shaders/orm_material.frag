
uniform float metallic;
uniform float roughness;

#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
uniform sampler2D metallicRoughnessTexture;
uniform mat3 metallicRoughnessTexTransform;
#endif

#ifdef USE_OCCLUSION_TEXTURE
uniform sampler2D occlusionTexture;
uniform mat3 occlusionTexTransform;
uniform float occlusionStrength;
#endif

layout (location = 0) out vec4 outColor;

void main()
{
    float metallic_factor = metallic;
    float roughness_factor = roughness;
#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
    vec2 t = texture(metallicRoughnessTexture, (metallicRoughnessTexTransform * vec3(uvs, 1.0)).xy).gb;
    roughness_factor *= t.x;
    metallic_factor *= t.y;
#endif

    float occlusion = 1.0;
#ifdef USE_OCCLUSION_TEXTURE
    occlusion = mix(1.0, texture(occlusionTexture, (occlusionTexTransform * vec3(uvs, 1.0)).xy).r, occlusionStrength);
#endif

    outColor = vec4(occlusion, roughness_factor, metallic_factor, 1.0);
}