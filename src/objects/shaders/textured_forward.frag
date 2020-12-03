
uniform bool use_uvs;
uniform sampler2D tex;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 pos;
in vec3 nor;
in vec2 uvs;

layout (location = 0) out vec4 out_color;

void main()
{
	vec3 n = normalize(gl_FrontFacing ? nor : -nor);
	float d = diffuse_intensity + specular_intensity*specular_power;
	if(use_uvs) {
		vec4 color = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
		out_color = vec4(d * color.rgb, color.a);
	} else {
    	out_color = vec4(d * triplanarMapping(tex, n, pos), 1.0);
	}
}