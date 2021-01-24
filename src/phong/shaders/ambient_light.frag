
uniform vec3 ambientColor;

layout (location = 0) out vec4 color;

void main()
{
    vec3 surfaceColor = get_surface_color();
    color = vec4(surfaceColor * ambientColor, 1.0);
}
