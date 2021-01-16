
uniform sampler2D tex;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

uniform BaseLight ambientLight;

in vec3 pos;
in vec3 nor;
in vec2 uvs;

layout (location = 0) out vec4 out_color;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
	vec4 color = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
	Surface surface = Surface(pos, normal, color.rgb, diffuse_intensity, specular_intensity, specular_power);
    out_color = vec4(calculate_ambient_light(ambientLight, surface), color.a);
}