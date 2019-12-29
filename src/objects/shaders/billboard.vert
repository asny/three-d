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
in vec2 uv_coordinate;

out vec2 uv;

void main()
{
    uv = uv_coordinate;
    vec3 dir = camera.position - (modelMatrix * vec4(0., 0., 0., 1.)).xyz;
    mat4 rotationMatrix = mat4(1.0);
    gl_Position = camera.viewProjection * modelMatrix * rotationMatrix * vec4(position, 1.);
}
