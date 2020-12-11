
uniform vec4 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

layout (std140) uniform DirectionalLightUniform
{
    DirectionalLight light;
};

in vec3 nor;
in vec3 pos;
in vec2 uvs;

layout (location = 0) out vec4 out_color;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
	Surface surface = Surface(pos, normal, color.rgb, diffuse_intensity, specular_intensity, specular_power);
    out_color = vec4(calculate_directional_light(light, surface), color.a);
}