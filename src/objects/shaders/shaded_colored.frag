
uniform bool use_texture;
uniform sampler2D tex;
uniform vec3 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;
layout (location = 3) out vec4 surface_parameters;

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
    out_color = vec4(use_texture ? triplanarMapping(tex, nor, pos) : color , 1.0);
    position = vec4(pos, 1.0);
    normal = vec4(nor, 1.0);
    surface_parameters = vec4(diffuse_intensity, specular_intensity, specular_power, 0.0);
}