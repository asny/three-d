uniform vec4 color;

#ifdef USE_TEXTURE
uniform sampler2D tex;
#endif

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = color;
    
    #ifdef USE_VERTEX_COLORS
    outColor *= col;
    #endif
    
    #ifdef USE_TEXTURE
    vec4 tex_color = texture(tex, uvs);
    outColor *= vec4(rgb_from_srgb(tex_color.rgb), tex_color.a);
    #endif

    outColor.rgb = srgb_from_rgb(outColor.rgb);
}