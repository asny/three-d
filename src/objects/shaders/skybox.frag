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

void main() {
    color = vec4(texture(texture0, coords).rgb, 1.0);
}
