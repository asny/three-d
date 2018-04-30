uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 Position;

out vec2 coords;

void main()
{
  coords = Position.xy;
  gl_Position = projectionMatrix * viewMatrix * vec4(Position, 1.0);
}
