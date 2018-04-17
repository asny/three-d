uniform mat4 ViewMatrix;

in vec3 Position;
in vec3 Color;

out vec3 col;

void main()
{
  col = Color;
  gl_Position = ViewMatrix * vec4(Position, 1.0);
}
