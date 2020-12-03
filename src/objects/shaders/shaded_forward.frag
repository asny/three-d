
uniform vec4 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 nor;
in vec3 pos;
in vec2 uvs;

layout (location = 0) out vec4 out_color;

void main()
{
	vec3 n = normalize(gl_FrontFacing ? nor : -nor);
	float d = diffuse_intensity + specular_intensity*specular_power;
    out_color = vec4(color.rgb * d, color.a);
}