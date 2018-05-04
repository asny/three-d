uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 Position;

out vec3 posWorld;

void main()
{
  posWorld = Position;
  gl_Position = projectionMatrix * viewMatrix * vec4(Position, 1.0);
}
