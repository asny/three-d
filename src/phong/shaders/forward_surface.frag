
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

#ifdef USE_COLOR_TEXTURE
uniform sampler2D tex;
#else 
uniform vec4 surfaceColor;
#endif

in vec3 pos;
in vec3 nor;

Surface get_surface()
{
    vec4 color;
#ifdef USE_COLOR_TEXTURE
    color = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
#else 
    color = surfaceColor;
#endif
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    return Surface(pos, normal, color, diffuse_intensity, specular_intensity, specular_power);
}