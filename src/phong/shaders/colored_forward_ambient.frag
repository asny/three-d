
uniform vec4 surfaceColor;
uniform vec3 ambientColor;

in vec3 nor;
in vec3 pos;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(surfaceColor.rgb * ambientColor, surfaceColor.a);
}