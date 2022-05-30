uniform sampler2DArray tex;

uniform mat4 view;
uniform int no_views;
in vec2 uvs;

layout (location = 0) out vec4 out_color;

void main()
{
    vec3 dir = normalize(vec3(view[0][2], 0.0, view[2][2]));
    float a = acos(dir.x);
    float angle = (dir.z > 0.0 ? a : 2.0 * PI - a) / (2.0 * PI);

    float layer = float(no_views) * clamp(angle, 0.0, 0.999);

    float index0 = floor(layer);
    float index1 = float((int(index0) + 1) % no_views);
    float frac = layer - index0;

    vec4 color0 = texture(tex, vec3(uvs.x, uvs.y, index0));
    color0.rgb = rgb_from_srgb(color0.rgb);
    vec4 color1 = texture(tex, vec3(uvs.x, uvs.y, index1));
    color1.rgb = rgb_from_srgb(color1.rgb);
    out_color = mix(color0, color1, frac);
    out_color = vec4(srgb_from_rgb(out_color.rgb), out_color.a);
    if(out_color.a < 0.5) {
        discard;
    }
}
