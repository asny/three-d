uniform mat3 textureTransform;

in vec3 position;

out vec2 uvs;

void main()
{
    uvs = (textureTransform * vec3(0.5 * position.x + 0.5, 0.5 * position.y + 0.5, 1.0)).xy;
    gl_Position = vec4(position, 1.0);
}