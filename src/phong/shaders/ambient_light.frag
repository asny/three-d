
layout (location = 0) out vec4 color;

uniform BaseLight ambientLight;

void main()
{
    color = vec4(calculate_ambient_light(ambientLight, get_surface()), 1.0);
}
