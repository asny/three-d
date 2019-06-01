in vec3 position;
in vec2 uv_coordinate;

out vec2 uv;

void main()
{
    uv = uv_coordinate;
    gl_Position = vec4(position, 1.0);
}
