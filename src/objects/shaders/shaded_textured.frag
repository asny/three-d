
uniform sampler2D texture0;

in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;

vec3 blendNormal(vec3 normal){
	vec3 blending = abs(normal);
	blending = normalize(max(blending, 0.00001));
	blending /= vec3(blending.x + blending.y + blending.z);
	return blending;
}

vec3 triplanarMapping (sampler2D tex, vec3 normal, vec3 position) {
    vec3 normalBlend = blendNormal(normal);
	vec3 xColor = texture(tex, 0.5 + 0.5*position.yz).rgb;
	vec3 yColor = texture(tex, 0.5 + 0.5*position.xz).rgb;
	vec3 zColor = texture(tex, 0.5 + 0.5*position.xy).rgb;

    return (xColor * normalBlend.x + yColor * normalBlend.y + zColor * normalBlend.z);
}

void main()
{
    color = vec4(triplanarMapping(texture0, nor, pos), 1.0);
    position = vec4(pos, 1.0);
    normal = vec4(nor, 1.0);
}
