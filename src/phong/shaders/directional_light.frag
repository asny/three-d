
layout (location = 0) out vec4 color;

layout (std140) uniform DirectionalLightUniform
{
    DirectionalLight light;
};

void main()
{
    Surface surface = get_surface();
    color = vec4(calculate_directional_light(light, surface.color, surface.position, surface.normal,
        surface.diffuse_intensity, surface.specular_intensity, surface.specular_power), 1.0);
}
