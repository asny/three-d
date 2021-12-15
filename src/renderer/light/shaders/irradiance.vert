
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

in vec3 position;

out vec3 pos;

void main()
{
    pos = position;
    gl_Position = camera.viewProjection * vec4(position, 1.0);
}