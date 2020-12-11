
layout (location = 0) out vec4 color;

void main()
{
    color = vec4(calculate_directional_light(get_surface()), 1.0);
}
