
uniform bool use_uvs;
uniform sampler2D tex;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 pos;
in vec3 nor;
in vec2 uvs;

layout (location = 0) out vec4 out_color;

vec3 blendNormal(vec3 normal){
	vec3 blending = abs(normal);
	blending = normalize(max(blending, 0.00001));
	blending /= vec3(blending.x + blending.y + blending.z);
	return blending;
}

vec3 triplanarMapping (sampler2D t, vec3 normal, vec3 position) {
    vec3 normalBlend = blendNormal(normal);
	vec3 xColor = texture(t, 0.5 + 0.5*position.yz).rgb;
	vec3 yColor = texture(t, 0.5 + 0.5*position.xz).rgb;
	vec3 zColor = texture(t, 0.5 + 0.5*position.xy).rgb;

    return (xColor * normalBlend.x + yColor * normalBlend.y + zColor * normalBlend.z);
}

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