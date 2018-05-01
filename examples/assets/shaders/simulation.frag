
uniform sampler2D indexToPosition;

in vec2 coords;

out vec4 fragmentColor;

void main()
{
    float c = texture(indexToPosition, coords).r;
    fragmentColor = vec4(c, c, c, 1.0f);
}
