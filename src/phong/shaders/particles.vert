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
in vec3 normal;
in vec2 uv_coordinates;

in vec3 start_position;
in vec3 start_velocity;

out vec3 pos;
out vec3 nor;
out vec2 uvs;

void main()
{
    vec3 p = start_position + start_velocity * time + 0.5 * acceleration * time * time;
    vec4 worldPosition = vec4(p + position, 1.);
    nor = normal;
    pos = worldPosition.xyz;
    uvs = uv_coordinates;
    gl_Position = camera.viewProjection * worldPosition;
}
