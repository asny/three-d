
in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = col/255.0;
}