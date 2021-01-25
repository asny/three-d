
uniform sampler2D colorMap;

in vec2 uv;

layout (location = 0) out vec4 outColor;

void main()
{
    vec4 color = vec4(0.0);

    for(int i = -5; i <= 5; i++) {
        for(int j = -5; j <= 5; j++) {
            float distSqr = j * j + i * i;
            if(distSqr < 25.0) {
                color += texture(colorMap, uv + vec2(i / 800.0, j / 800.0)) / (1.0 + distSqr);
            }
        }
    }

    outColor = vec4(color.rgb, 1.0);
}