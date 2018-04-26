uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 Position;
in vec3 Color;

out vec3 col;
out vec2 coords;

void main()
{
  col = Color;
  coords = Position.xy;
  gl_Position = projectionMatrix * viewMatrix * vec4(Position, 1.0);
}
