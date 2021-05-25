uniform vec3 ambientColor;

layout (location = 0) out vec4 outColor;

void main()
{
    Surface surface = get_surface(); 
    outColor = surface.color;
    outColor.rgb *= ambientColor;
    calculate_lighting(outColor, surface);
    outColor.rgb = srgb_from_rgb(outColor.rgb);
}