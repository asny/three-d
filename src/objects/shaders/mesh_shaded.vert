uniform mat4 modelMatrix;
uniform mat4 normalMatrix;

in vec3 position;
in vec3 normal;

out vec3 pos;
out vec3 nor;

uniform Camera
{
    mat4 viewProjection;
};

void main()
{
    pos = (modelMatrix * vec4(position, 1.)).xyz;
    nor = mat3(normalMatrix) * normal;
    gl_Position = viewProjection * modelMatrix * vec4(position, 1.0);
}
