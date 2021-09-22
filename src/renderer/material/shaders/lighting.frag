
#ifdef DEFERRED 

uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;
uniform mat4 viewProjectionInverse;

#else

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

#endif

layout (location = 0) out vec4 outColor;

void main()
{
#ifdef DEFERRED 

    float depth = texture(depthMap, vec3(uv,0)).r;
    if(depth > 0.99999)
    {
        discard;
    }
    gl_FragDepth = depth;

    vec3 position = world_pos_from_depth(viewProjectionInverse, depth, uv);
   	
    vec4 c = texture(gbuffer, vec3(uv, 0));
    vec4 surface_color = vec4(c.rgb, 1.0);
    float metallic_factor = c.w;

    vec4 n = texture(gbuffer, vec3(uv, 1));
    vec2 n2 = n.xy*2.0 - 1.0;
    float z = 1.0 - n2.x * n2.x - n2.y * n2.y;
    if (z > 0.0001) {
        z = sqrt(z);
    }
    vec3 normal = normalize(vec3(n2.x, n2.y, z));
    float roughness_factor = n.w;
    float occlusion = n.z;

#else 

    vec4 surface_color = albedo;
#ifdef USE_ALBEDO_TEXTURE
    vec4 c = texture(albedoTexture, uvs);
    surface_color *= vec4(rgb_from_srgb(c.rgb), c.a);
#endif

    float occlusion = 1.0;
#ifdef USE_OCCLUSION_TEXTURE
    occlusion = mix(1.0, texture(occlusionTexture, uvs).r, occlusionStrength);
#endif

    float metallic_factor = metallic;
    float roughness_factor = roughness;
#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
    vec2 t = texture(metallicRoughnessTexture, uvs).gb;
    metallic_factor *= t.y;
    roughness_factor *= t.x;
#endif

    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
#ifdef USE_NORMAL_TEXTURE
    mat3 tbn = basis(normal);
    normal = tbn * ((2.0 * texture(normalTexture, uvs).xyz - 1.0) * vec3(normalScale, normalScale, 1.0));
#endif

    vec3 position = pos;

#endif

    outColor.rgb = srgb_from_rgb(calculate_lighting(surface_color.rgb, position, normal, metallic_factor, roughness_factor, occlusion));
    outColor.a = surface_color.a;
}