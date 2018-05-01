uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 Position;
in vec3 Color;

out vec3 col;
out vec3 posWorld;

void main()
{
  col = Color;
  posWorld = Position;
  gl_Position = projectionMatrix * viewMatrix * vec4(Position, 1.0);
}
