
uniform sampler2D tex;

in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = texture(tex, uvs);
}