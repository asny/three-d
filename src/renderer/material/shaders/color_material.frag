uniform vec4 color;

#ifdef USE_TEXTURE
in vec2 uvs;
uniform sampler2D tex;
#endif

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = color;
    #ifdef USE_TEXTURE
    outColor *= texture(tex, uvs);
    #endif
}