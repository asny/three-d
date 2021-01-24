
uniform sampler2D colorMap;

in vec2 uv;

layout (location = 0) out vec4 outColor;

void main()
{
    vec4 color = texture(colorMap, uv);
    outColor = vec4(color.rgb, 1.0);
}