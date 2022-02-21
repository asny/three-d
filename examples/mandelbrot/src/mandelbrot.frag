
in vec3 pos;

layout (location = 0) out vec4 color;

vec2 fun(vec2 z, vec2 c)
{
    return vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + c;
}

void main()
{
    int m = 128;
    vec2 z = vec2(0);
    for(int i = 0; i < m; i++) {
        z = fun(z, pos.xy);
        if(length(z) >= 2.0) {
            float t = float(i);
            color = vec4(smoothstep(0.0, 16.0, t) - smoothstep(32.0, float(m), t),
                1.0 - smoothstep(16.0, 32.0, t),
                1.0 - smoothstep(0.0, 16.0, t), 1.0);
            return;
        }
    }
    color = vec4(0.0, 0.0, 0.0, 1.0f);
}
