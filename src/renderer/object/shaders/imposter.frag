uniform sampler2DArray tex;

uniform mat4 view;
uniform int no_views;
in vec2 uvs;

layout (location = 0) out vec4 outColor;

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
    vec4 color1 = texture(tex, vec3(uvs.x, uvs.y, index1));
    outColor = mix(color0, color1, frac);
    if(outColor.a < 0.5) {
        discard;
    }
    outColor.rgb = tone_mapping(outColor.rgb);
    outColor.rgb = color_mapping(outColor.rgb);
}
