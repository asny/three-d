uniform samplerCube texture0;

layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

in vec3 coords;

layout (location = 0) out vec3 color;
layout (location = 1) out vec3 normal;
layout (location = 2) out vec3 surface_parameters;

void main() {
    color = texture(texture0, coords).rgb;
    normal = 0.5 * normalize(-coords) + 1.0;
    surface_parameters = vec3(0.0, 0.0, 0.0);
}
