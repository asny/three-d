uniform sampler2DArray tex;

in vec2 uv;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 out_normal;

void main()
{
    vec4 color = texture(tex, vec3(uv, 0.0));
    vec4 normal = texture(tex, vec3(uv, 1.0));
    if(color.a < 0.01 && normal.a < 0.01) { // No diffuse or specular intensity. Test depth instead?
        discard;
    }
    out_color = color;
    out_normal = normal;
    // Maybe update depth as well?
}
