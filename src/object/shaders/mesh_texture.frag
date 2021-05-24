
uniform sampler2D tex;

in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    vec4 col = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
    outColor = vec4(srgb_from_rgb(col.rgb), col.a);
}