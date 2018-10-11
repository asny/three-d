uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
uniform mat4 normalMatrix;

in vec3 position;
in vec3 normal;

out vec3 pos;
out vec3 nor;

void main()
{
    pos = (modelMatrix * vec4(position, 1.)).xyz;
    nor = mat3(normalMatrix) * normal;
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
}
