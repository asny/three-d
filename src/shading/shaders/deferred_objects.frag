
uniform float metallic;
uniform float roughness;

#ifdef USE_COLOR_TEXTURE
uniform sampler2D tex;
#else 
uniform vec4 surfaceColor;
#endif

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void main()
{
	vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec4 color;
#ifdef USE_COLOR_TEXTURE
    color = texture(tex, vec2(uvs.x, 1.0 - uvs.y));
#else 
    color = surfaceColor;
#endif
    out_color = vec4(color.rgb, metallic);
    out_normal = vec4(0.5 * normal + 0.5, roughness);
}