uniform sampler2D colorMap;
uniform sampler2D normalMap;

in vec2 uv;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 normal;

void main()
{
    float diffuse_intensity = 0.5;
    float specular_power = 0.0;
    float specular_intensity = 0.0;

    out_color = vec4(texture(colorMap, uv).rgb, diffuse_intensity);
	int intensity = int(floor(specular_intensity * 15.0));
	int power = int(floor(specular_power*0.5));
    vec3 nor = normalize(texture(normalMap, uv).xyz);
    normal = vec4(0.5 * nor + 0.5, float(power << 4 | intensity)/255.0);
    // Maybe update depth as well?
}
