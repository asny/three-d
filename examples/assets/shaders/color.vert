uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;
in vec3 color;

out vec3 col;

void main()
{
  col = color;
  gl_Position = projectionMatrix * viewMatrix * vec4(position, 1.0);
}
