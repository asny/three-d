uniform sampler2DArray tex;

uniform int no_views;
in vec2 uv;
in float t;
in vec2 cs;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void main()
{
    float layer = float(no_views) * t;

    float index = floor(layer);
    vec4 color0 = texture(tex, vec3(uv, index));
    vec4 normal0 = texture(tex, vec3(uv, float(no_views) + index));
    if(color0.a < 0.01 && normal0.a < 0.01) { // No diffuse or specular intensity. Test depth instead? Dithering?
        discard;
    }

    index = float((int(index) + 1) % no_views);
    vec4 color1 = texture(tex, vec3(uv, index));
    vec4 normal1 = texture(tex, vec3(uv, float(no_views) + index));
    if(color1.a < 0.01 && normal1.a < 0.01) { // No diffuse or specular intensity. Test depth instead? Dithering?
        discard;
    }

    out_color = mix(color0, color1, fract(layer));
    out_normal = mix(normal0, normal1, fract(layer));
    out_normal.xyz = 2.0 * out_normal.xyz - 1.0;
    out_normal.xyz = 0.5 + 0.5 * normalize(vec3(cs.x * out_normal.x + cs.y * out_normal.z, out_normal.y, -cs.y * out_normal.x + cs.x * out_normal.z));
    // Maybe update depth as well?
}
