
uniform bool use_texture;
uniform sampler2D tex;
uniform vec3 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 normal;

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
    out_color = vec4(use_texture ? triplanarMapping(tex, n, pos) : color, diffuse_intensity);
	int intensity = int(floor(specular_intensity * 15.0));
	int power = int(floor(specular_power*0.5));
    normal = vec4(0.5 * n + 0.5, float(power << 4 | intensity)/255.0);
}