uniform vec3 ambientColor;

#ifdef DEFERRED 

uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;
uniform mat4 viewProjectionInverse;

#else

uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

#ifdef USE_COLOR_TEXTURE
uniform sampler2D tex;
#else 
uniform vec4 surfaceColor;
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
   	vec4 c = texture(gbuffer, vec3(uv, 0));
    vec4 surface_color = vec4(c.rgb, 1.0);
    vec3 position = world_pos_from_depth(viewProjectionInverse, depth, uv);
    vec4 n = texture(gbuffer, vec3(uv, 1));
    vec3 normal = normalize(n.xyz*2.0 - 1.0);
    float diffuse_intensity = c.w;
    int t = int(floor(n.w*255.0));
    float specular_intensity = float(t & 15) / 15.0;
    float specular_power = 2.0 * float((t & 240) >> 4);

#else 

    vec4 surface_color;
#ifdef USE_COLOR_TEXTURE
    surface_color = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
#else 
    surface_color = surfaceColor;
#endif
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec3 position = pos;

#endif

    // Material parameters
    // TODO
    float metallic = 0.5;
    float roughness = 0.5;

    outColor.rgb = surface_color.rgb * mix(ambientColor, vec3(0.0), metallic);
    calculate_lighting(outColor, surface_color.rgb, position, normal, metallic, roughness);
    outColor = vec4(srgb_from_rgb(outColor.rgb), surface_color.a);
}