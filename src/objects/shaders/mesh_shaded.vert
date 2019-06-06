uniform mat4 modelMatrix;
uniform mat4 normalMatrix;

in vec3 position;
in vec3 normal;

out vec3 pos;
out vec3 nor;

uniform Matrices
{
    mat4 view;
    mat4 projection;
};

void main()
{
    pos = (modelMatrix * vec4(position, 1.)).xyz;
    nor = mat3(normalMatrix) * normal;
    gl_Position = projection * view * modelMatrix * vec4(position, 1.0);
}
