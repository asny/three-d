uniform vec4 color;

#ifdef USE_TEXTURE
uniform sampler2D tex;
#endif

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = color;
    
    #ifdef USE_VERTEX_COLORS
    outColor *= vec4(srgb_from_rgb(col.rgb), col.a);
    #endif
    
    #ifdef USE_TEXTURE
    outColor *= texture(tex, uvs);
    #endif
}