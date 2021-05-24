uniform sampler2DArray tex;

uniform int no_views;
in vec2 uv;
in float t;

layout (location = 0) out vec4 out_color;

void main()
{
    float layer = float(no_views) * clamp(t, 0.0, 0.999);

    float index0 = floor(layer);
    float index1 = float((int(index0) + 1) % no_views);
    float frac = layer - index0;

    vec4 color0 = texture(tex, vec3(uv, index0));
    color0.rgb = rgb_from_srgb(color0.rgb);
    vec4 color1 = texture(tex, vec3(uv, index1));
    color1.rgb = rgb_from_srgb(color1.rgb);
    out_color = mix(color0, color1, frac);
    out_color = vec4(srgb_from_rgb(out_color.rgb), out_color.a);
    if(out_color.a < 0.5) {
        discard;
    }
}
