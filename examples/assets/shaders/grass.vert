uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;
in vec3 root_position;

out vec3 pos;
out vec3 nor;

void main()
{
  vec4 p = projectionMatrix * viewMatrix * vec4(root_position + position, 1.0);
  gl_Position = p;
  pos = p.xyz;
  nor = vec3(0.0, 1.0, 0.0);
}
