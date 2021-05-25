
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

#ifdef USE_COLOR_TEXTURE
uniform sampler2D tex;
#else 
uniform vec4 surfaceColor;
#endif

in vec3 pos;
in vec3 nor;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void write(vec3 normal, vec3 color, float diffuse_intensity, float specular_intensity, float specular_power)
{
    out_color = vec4(color, diffuse_intensity);
	int intensity = int(floor(specular_intensity * 15.0));
	int power = int(floor(clamp(specular_power, 0.0, 30.0)*0.5));
    out_normal = vec4(0.5 * normal + 0.5, float(power << 4 | intensity)/255.0);
}

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec4 color;
#ifdef USE_COLOR_TEXTURE
    color = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
#else 
    color = surfaceColor;
#endif
	write(normal, color.rgb, diffuse_intensity, specular_intensity, specular_power);
}