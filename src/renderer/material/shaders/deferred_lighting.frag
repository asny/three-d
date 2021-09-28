
uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;
uniform mat4 viewProjectionInverse;

in vec2 uv;

layout (location = 0) out vec4 outColor;

void main()
{
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

    outColor.rgb = srgb_from_rgb(calculate_lighting(surface_color.rgb, position, normal, metallic_factor, roughness_factor, occlusion));
    outColor.a = surface_color.a;
}