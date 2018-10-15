
uniform vec3 color;

const float diffuseIntensity = 0.2f;
const float specularIntensity = 0.2f;
const float specularPower = 5.f;

in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;
layout (location = 3) out vec4 surface_parameters;

void main()
{
    position = vec4(pos, 1.0);
    out_color = vec4(color, 1.0);
    normal = vec4(nor, 1.0);
    surface_parameters = vec4(diffuseIntensity, specularIntensity, specularPower, 0.0);
}
