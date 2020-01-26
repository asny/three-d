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
    vec4 color0 = texture(tex, vec3(uv, floor(layer)));
    vec4 color1 = texture(tex, vec3(uv, int(ceil(layer)) % no_views));
    vec4 normal0 = texture(tex, vec3(uv, float(no_views) + floor(layer)));
    normal0.xyz = 2.0 * normal0.xyz - 1.0;
    vec4 normal1 = texture(tex, vec3(uv, float(no_views + int(ceil(layer)) % no_views)));
    normal1.xyz = 2.0 * normal1.xyz - 1.0;
    if(color0.a < 0.01 && normal0.a < 0.01 || color1.a < 0.01 && normal1.a < 0.01) { // No diffuse or specular intensity. Test depth instead? Dithering?
        discard;
    }
    out_color = mix(color0, color1, fract(layer));
    out_normal = normalize(mix(normal0, normal1, fract(layer)));
    out_normal.xyz = 0.5 + 0.5 * normalize(vec3(cs.x * out_normal.x + cs.y * out_normal.z, out_normal.y, -cs.y * out_normal.x + cs.x * out_normal.z));
    // Maybe update depth as well?
}
