uniform sampler2DArray tex;

uniform int no_views;
in vec2 uv;
in float t;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void main()
{
    float layer = float(no_views) * t;
    vec4 color0 = texture(tex, vec3(uv, floor(layer)));
    vec4 color1 = texture(tex, vec3(uv, int(ceil(layer)) % no_views));
    vec4 color = mix(color0, color1, fract(layer));
    vec4 normal0 = texture(tex, vec3(uv, float(no_views) + floor(layer)));
    vec4 normal1 = texture(tex, vec3(uv, float(no_views + int(ceil(layer)) % no_views)));
    vec4 normal = mix(normal0, normal1, fract(layer));
    if(color0.a < 0.01 && normal0.a < 0.01 || color1.a < 0.01 && normal1.a < 0.01) { // No diffuse or specular intensity. Test depth instead? Dithering?
        discard;
    }
    else {

        out_color = color;
        out_normal = normal;
    }
    // Maybe update depth as well?
}
