
in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(pos, 1.0);
}