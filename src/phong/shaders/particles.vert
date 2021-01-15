layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

in vec3 position;
in vec3 normal;
in vec2 uv_coordinates;

in vec3 start_pos;

out vec3 pos;
out vec3 nor;
out vec2 uvs;

void main()
{
    vec4 worldPosition = vec4(start_pos + position, 1.);
    nor = normal;
    pos = worldPosition.xyz;
    uvs = uv_coordinates;
    gl_Position = camera.viewProjection * worldPosition;
}
