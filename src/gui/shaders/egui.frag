
uniform sampler2D u_sampler;

in vec4 v_rgba;
in vec2 v_tc;

layout (location = 0) out vec4 color;

void main() {
    // The texture is in linear space so we need to decode here
    vec4 texture_rgba = texture(u_sampler, v_tc);
    texture_rgba.rgb = rgb_from_srgb(texture_rgba.rgb);
    /// Multiply vertex color with texture color (in linear space).
    color = v_rgba * texture_rgba;
    // We must gamma-encode again since WebGL doesn't support linear blending in the framebuffer.
    color.rgb = srgb_from_rgb(color.rgb);
    // WebGL doesn't support linear blending in the framebuffer,
    // so we apply this hack to at least get a bit closer to the desired blending:
    color.a = pow(color.a, 1.6); // Empiric nonsense
}