
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 local2worldX;
in vec3 local2worldY;
in vec3 local2worldZ;
in vec3 translation;
in vec3 normalMatrixX;
in vec3 normalMatrixY;
in vec3 normalMatrixZ;


in vec3 position;

out vec3 pos;
out vec3 nor;

mat4 local2world()
{
    return mat4(local2worldX.x, local2worldX.y,  local2worldX.z, 0.0,
                local2worldY.x, local2worldY.y,  local2worldY.z, 0.0,
                local2worldZ.x, local2worldZ.y,  local2worldZ.z, 0.0,
                translation.x, translation.y, translation.z, 1.0);
}

mat3 normalMatrix()
{
    return mat3(normalMatrixX.x, normalMatrixX.y,  normalMatrixX.z,
                normalMatrixY.x, normalMatrixY.y,  normalMatrixY.z,
                normalMatrixZ.x, normalMatrixZ.y,  normalMatrixZ.z);
}

void main()
{
    pos = (local2world() * vec4(position, 1.0)).xyz;
    nor = normalize(normalMatrix() * vec3(0.0, position.y, position.z));
    gl_Position = projectionMatrix * viewMatrix * vec4(pos, 1.0);
}