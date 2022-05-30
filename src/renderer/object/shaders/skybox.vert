
uniform mat4 view;
uniform mat4 projection;

in vec3 position;

out vec3 coords;

void main()
{
    coords = position;
    gl_Position = (projection * mat4(mat3(view)) * vec4(position, 1.)).xyww;
}