
in vec3 nor;

layout (location = 0) out vec4 outColor;

void main()
{
    vec3 col = 0.5 + 0.5 * nor;
    outColor = vec4(srgb_from_rgb(col), 1.0);
}