
uniform bool use_uvs;
uniform sampler2D tex;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 pos;
in vec3 nor;
in vec2 uvs;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec3 color = use_uvs ? texture(tex, vec2(uvs.x, 1.0 - uvs.y)).rgb: triplanarMapping(tex, normal, pos);
	write(normal, color, diffuse_intensity, specular_intensity, specular_power);
}