
uniform sampler2D tex;

in vec3 col;
in vec2 coords;

out vec4 fragmentColor;

void main()
{
    float c = texture(tex, coords).r;
    fragmentColor = vec4(c * col, 1.0f);
}
