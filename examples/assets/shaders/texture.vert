uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;

out vec3 posWorld;

void main()
{
  posWorld = position;
  gl_Position = projectionMatrix * viewMatrix * vec4(position, 1.0);
}
