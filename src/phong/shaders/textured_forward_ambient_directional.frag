
uniform sampler2D shadowMap;
uniform sampler2D tex;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

uniform vec3 ambientColor;

layout (std140) uniform DirectionalLightUniform
{
    DirectionalLight light;
};

in vec3 pos;
in vec3 nor;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
	vec4 surfaceColor = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
	vec3 directional_color = calculate_directional_light(light, surfaceColor.rgb, pos, normal,
		diffuse_intensity, specular_intensity, specular_power, shadowMap);
    outColor = vec4(surfaceColor.rgb * ambientColor + directional_color, surfaceColor.a);
}