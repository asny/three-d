
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position0;
in vec3 position1;

in vec3 position;

out vec3 pos;
out vec3 nor;

void main()
{
    pos = position;
    pos.x *= distance(position1, position0);
    pos.y *= 0.1;
    pos.z *= 0.1;

    pos += position0;
    nor = vec3(0.0, 1.0, 0.0);

    gl_Position = projectionMatrix * viewMatrix * vec4(pos, 1.0);
}
