uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;
in vec3 offset;

void main()
{
  gl_Position = projectionMatrix * viewMatrix * vec4(position + offset, 1.0);
}
