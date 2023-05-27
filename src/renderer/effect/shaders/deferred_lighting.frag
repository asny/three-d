
uniform mat4 viewProjectionInverse;
uniform float zNear;
uniform float zFar;
uniform vec3 cameraPosition;
uniform int debug_type;

in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    float depth = sample_depth(uvs);
    if(depth > 0.99999)
    {
        discard;
    }
    gl_FragDepth = depth;

    vec3 position = world_pos_from_depth(viewProjectionInverse, depth, uvs);
   	
    vec4 c = sample_layer(uvs, 0);
    vec4 surface_color = vec4(c.rgb, 1.0);
    float metallic_factor = c.w;

    vec4 n = sample_layer(uvs, 1);
    vec2 n2 = n.xy*2.0 - 1.0;
    float z = 1.0 - n2.x * n2.x - n2.y * n2.y;
    if (z > 0.0001) {
        z = sqrt(z);
    }
    vec3 normal = normalize(vec3(n2.x, n2.y, (int(floor(n.z * 255.0)) & 128) == 128 ? z: -z));
    float roughness_factor = n.w;
    float occlusion = float(int(floor(n.z * 255.0)) & 127) / 127.0;
    vec3 total_emissive = sample_layer(uvs, 2).rgb;

    if(debug_type == 0) // Position
    {
        outColor = vec4(position, 1.);
    }
    else if(debug_type == 1) // Normal
    {
        outColor = vec4(normal * 0.5 + 0.5, 1.);
    }
    else if(debug_type == 2) // Color
    {
        outColor = vec4(srgb_from_rgb(surface_color.rgb), surface_color.a);
    }
    else if(debug_type == 3) // Depth
    {
        float dist = (distance(position, cameraPosition) - zNear) / (zFar - zNear);
        outColor = vec4(dist, dist, dist, 1.);
    }
    else if(debug_type == 4) // ORM
    {
        outColor = vec4(occlusion, roughness_factor, metallic_factor, 1.0);
    }
    else if(debug_type == 5) // UV
    {
        outColor = vec4(uvs, 0., 1.);
    }
    else { // None
        outColor.rgb = total_emissive + calculate_lighting(cameraPosition, surface_color.rgb, position, normal, metallic_factor, roughness_factor, occlusion);
        outColor.rgb = reinhard_tone_mapping(outColor.rgb);
        outColor.rgb = srgb_from_rgb(outColor.rgb);
        outColor.a = surface_color.a;
    }
}