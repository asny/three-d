uniform sampler2DArray tex;

uniform int no_views;
in vec2 uvs;

layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

layout (location = 0) out vec4 out_color;

void main()
{
    vec3 dir = normalize(vec3(camera.view[0][2], 0.0, camera.view[2][2]));
    float c = dot(dir, vec3(0.0, 0.0, 1.0));
    float s = cross(dir, vec3(0.0, 0.0, 1.0)).y;
    mat3 rot = mat3(c, 0.0, s,
                0.0,  1.0, 0.0,
                -s,  0.0,  c);
    float angle = mod((s > 0.0 ? acos(c) : 2.0 * 3.1415926 - acos(c)), 2.0 * 3.1415926);
    float t = angle / (2.0 * 3.1415926);

    float layer = float(no_views) * clamp(t, 0.0, 0.999);

    float index0 = floor(layer);
    float index1 = float((int(index0) + 1) % no_views);
    float frac = layer - index0;

    vec4 color0 = texture(tex, vec3(uvs, index0));
    color0.rgb = rgb_from_srgb(color0.rgb);
    vec4 color1 = texture(tex, vec3(uvs, index1));
    color1.rgb = rgb_from_srgb(color1.rgb);
    out_color = mix(color0, color1, frac);
    out_color = vec4(srgb_from_rgb(out_color.rgb), out_color.a);
    if(out_color.a < 0.5) {
        discard;
    }
}
