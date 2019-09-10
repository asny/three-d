uniform mat4 modelMatrix;
uniform mat4 normalMatrix;

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

out vec3 pos;
out vec3 nor;

void main()
{
    pos = (modelMatrix * vec4(position, 1.)).xyz;
    nor = mat3(normalMatrix) * normal;
    gl_Position = camera.viewProjection * modelMatrix * vec4(position, 1.0);
}
