#version 100
precision mediump float;

attribute vec3 Position;
attribute vec3 Color;

varying vec3 col;

void main()
{
  col = Color;
  gl_Position = vec4(Position, 1.0);
}
