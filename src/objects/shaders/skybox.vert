uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;

out vec3 coords;

void main()
{
    coords = position;
    gl_Position = (projectionMatrix * mat4(mat3(viewMatrix)) * vec4(position, 1.)).xyww;
}