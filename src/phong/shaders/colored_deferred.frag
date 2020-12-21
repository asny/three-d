
uniform vec4 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 nor;
in vec3 pos;
in vec2 uvs;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
	write(normal, color.rgb, diffuse_intensity, specular_intensity, specular_power);
}