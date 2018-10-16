uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
uniform mat4 normalMatrix;

in vec3 position;
in vec3 normal;
in vec2 uv_coordinate;

out vec3 pos;
out vec3 nor;
out vec2 uv;

void main()
{
    pos = (modelMatrix * vec4(position, 1.)).xyz;
    nor = mat3(normalMatrix) * normal;
    uv = uv_coordinate;
    gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
}
