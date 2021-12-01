
in vec3 nor;

#ifdef USE_TEXTURE
uniform sampler2D normalTexture;
uniform float normalScale;
#endif

layout (location = 0) out vec4 outColor;

void main()
{

    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
#ifdef USE_TEXTURE
    mat3 tbn = basis(normal);
    normal = tbn * ((2.0 * texture(normalTexture, uvs).xyz - 1.0) * vec3(normalScale, normalScale, 1.0));
#endif
    outColor = vec4(0.5 + 0.5 * normal, 1.0);
}