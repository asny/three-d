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
in vec2 uv_coordinates;

out vec3 pos;
out vec3 nor;
out vec2 uvs;

in vec4 row1;
in vec4 row2;
in vec4 row3;

void main()
{
    mat4 transform;
    transform[0] = vec4(row1.x, row2.x, row3.x, 0.0);
    transform[1] = vec4(row1.y, row2.y, row3.y, 0.0);
    transform[2] = vec4(row1.z, row2.z, row3.z, 0.0);
    transform[3] = vec4(row1.w, row2.w, row3.w, 1.0);

    vec4 worldPosition = modelMatrix * transform * vec4(position, 1.);
    nor = mat3(normalMatrix) * mat3(transpose(inverse(transform))) * normal;
    pos = worldPosition.xyz;
    uvs = uv_coordinates;
    gl_Position = camera.viewProjection * worldPosition;
}