#version 300 es
precision mediump float;

in vec3 col;

out vec4 fragmentColor;

void main()
{
  fragmentColor = vec4(col, 1.0f);
}
