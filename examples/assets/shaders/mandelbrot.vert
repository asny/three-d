uniform mat4 modelMatrix;

layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

in vec3 position;

out vec2 pos;

void main()
{
    vec4 worldPosition = modelMatrix * vec4(position, 1.);
    pos = worldPosition.xy;
    gl_Position = camera.viewProjection * worldPosition;
}
