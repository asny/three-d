uniform sampler2D image;

in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = vec4(max(uvs.x, 1.0 - uvs.x), uvs.y, 1.0 - uvs.y, texture(image, uvs).g);
}