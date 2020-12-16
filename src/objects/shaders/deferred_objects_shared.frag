
layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void write(vec3 normal, vec3 color, float diffuse_intensity, float specular_intensity, float specular_power)
{
    out_color = vec4(color, diffuse_intensity);
	int intensity = int(floor(specular_intensity * 15.0));
	int power = int(floor(clamp(specular_power, 0.0, 30.0)*0.5));
    out_normal = vec4(0.5 * normal + 0.5, float(power << 4 | intensity)/255.0);
}