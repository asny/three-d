
layout (location = 0) out vec4 color;

layout (std140) uniform DirectionalLightUniform
{
    DirectionalLight light;
};

void main()
{
    color = vec4(calculate_directional_light(light, get_surface()), 1.0);
}
