uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 Position;
in vec3 Color;

out vec3 col;

void main()
{
  col = Color;
  gl_Position = projectionMatrix * viewMatrix * vec4(Position, 1.0);
}
