
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
uniform float scale;

in vec3 translation;

in vec3 position;

out vec3 pos;
out vec3 nor;

void main()
{
    pos = scale * position + translation;
    nor = normalize(position);
    gl_Position = projectionMatrix * viewMatrix * vec4(pos, 1.0);
}