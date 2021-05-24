
in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(srgb_from_rgb(vec3(uvs, 0.0)), 1.0);
}