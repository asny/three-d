uniform vec4 surfaceColor;

#ifdef USE_TEXTURE
uniform sampler2D tex;
uniform mat3 textureTransformation;
#endif

in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    outColor = surfaceColor * col;
    
    #ifdef USE_TEXTURE
    outColor *= texture(tex, (textureTransformation * vec3(uvs, 1.0)).xy);
    #endif

    outColor.rgb = color_mapping(outColor.rgb);
}