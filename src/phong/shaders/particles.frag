
uniform vec4 color;
uniform vec3 ambientColor;
uniform float ambientIntensity;

layout (location = 0) out vec4 out_color;

void main()
{
    out_color = vec4(color.rgb * ambientColor * ambientIntensity, color.a);
}