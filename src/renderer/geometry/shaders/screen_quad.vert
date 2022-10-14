uniform mat3 textureTransform;

in vec3 position;
in vec2 uv_coordinates;

out vec2 uv;

void main()
{
    uv = (textureTransform * vec3(uv_coordinates, 1.0)).xy;
    gl_Position = vec4(position, 1.0);
}