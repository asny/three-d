uniform sampler2DArray tex;

in vec2 uv;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 normal;

void main()
{
    vec4 color = texture(tex, vec3(uv, 0.0));
    if(color.a < 0.01) { // Test depth instead
        discard;
    }
    out_color = color;
    normal = texture(tex, vec3(uv, 1.0));
    // Maybe update depth as well?
}
