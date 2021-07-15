
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
    vec4 surface_color = vec4(rgb_from_srgb(c.rgb), 1.0);
    float metallic = c.w;

    vec4 n = texture(gbuffer, vec3(uv, 1));
    vec3 normal = normalize(n.xyz*2.0 - 1.0);
    float roughness = n.w;

#else 

    vec4 surface_color;
#ifdef USE_ALBEDO_TEXTURE
    vec4 c = texture(albedoTexture, vec2(uvs.x, 1.0 - uvs.y));
    surface_color = vec4(rgb_from_srgb(albedo.rgb * c.rgb), albedo.a * c.a);
#else 
    surface_color = vec4(rgb_from_srgb(albedo.rgb), albedo.a);
#endif
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec3 position = pos;

#endif

    outColor.rgb = srgb_from_rgb(calculate_lighting(surface_color.rgb, position, normal, metallic, roughness));
    outColor.a = surface_color.a;
}