uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;
in vec3 root_position;

void main()
{
  gl_Position = projectionMatrix * viewMatrix * vec4(root_position + position, 1.0);
}
