
uniform float metallic;
uniform float roughness;

uniform vec4 albedo;
#ifdef USE_ALBEDO_TEXTURE
uniform sampler2D tex;
#endif

#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
uniform sampler2D metallicRoughnessTexture;
#endif

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec4 color;
#ifdef USE_ALBEDO_TEXTURE
    color = albedo * texture(tex, vec2(uvs.x, 1.0 - uvs.y));
#else 
    color = albedo;
#endif

    float metallic_factor = metallic;
    float roughness_factor = roughness;
#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
    vec2 t = texture(metallicRoughnessTexture, vec2(uvs.x, 1.0 - uvs.y)).xy;
    metallic_factor *= t.x;
    roughness_factor *= t.y;
#endif

    out_color = vec4(color.rgb, metallic_factor);
    out_normal = vec4(0.5 * normal + 0.5, roughness_factor);
}