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

layout (location = 0) out vec4 color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;

void main() {
    color = texture(texture0, coords);
    position = vec4(camera.position + normalize(coords) * 100.f, 1.0);
    normal = vec4(-coords, 1.0);
}
