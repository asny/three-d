
uniform vec4 color;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(srgb_from_rgb(color.rgb), color.a);
}