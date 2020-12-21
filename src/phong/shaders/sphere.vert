
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform mat4 modelMatrix;
uniform float scale;

in vec3 translation;

in vec3 position;

out vec3 pos;
out vec3 nor;

void main()
{
    pos = scale * position + translation;
    nor = normalize(position);
    gl_Position = camera.viewProjection * modelMatrix * vec4(pos, 1.0);
}