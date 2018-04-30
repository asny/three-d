
uniform sampler2D tex;

in vec2 coords;

out vec4 fragmentColor;

void main()
{
    float c = texture(tex, coords).r;
    fragmentColor = vec4(c, c, c, 1.0f);
}
