uniform mat4 modelMatrix;
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform float time;
uniform vec3 acceleration;

in vec3 position;
in vec2 uv_coordinates;

in vec3 start_position;
in vec3 start_velocity;

out vec2 uvs;

void main()
{
    uvs = uv_coordinates;
    vec3 p = start_position + start_velocity * time + 0.5 * acceleration * time * time;
    gl_Position = camera.projection * (camera.view * modelMatrix * vec4(p, 1.0) + vec4(position, 0.0));
}
