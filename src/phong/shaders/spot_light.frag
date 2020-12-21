
layout (location = 0) out vec4 color;

layout (std140) uniform SpotLightUniform
{
    SpotLight light;
};

void main()
{
    color = vec4(calculate_spot_light(light, get_surface()), 1.0);
}
