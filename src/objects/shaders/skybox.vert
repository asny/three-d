
uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
} camera;

in vec3 position;

out vec3 coords;

void main()
{
    coords = position;
    gl_Position = (camera.projection * mat4(mat3(camera.view)) * vec4(position, 1.)).xyww;
}