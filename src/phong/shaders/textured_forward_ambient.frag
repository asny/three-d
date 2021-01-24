
uniform sampler2D tex;

uniform vec3 ambientColor;

in vec3 pos;
in vec3 nor;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
	vec4 surfaceColor = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
    outColor = vec4(surfaceColor.rgb * ambientColor, surfaceColor.a);
}