
uniform vec4 surfaceColor;
uniform vec3 ambientColor;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(surfaceColor.rgb * ambientColor, surfaceColor.a);
}