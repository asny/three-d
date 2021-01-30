
uniform vec4 color;

in vec2 uvs;

layout (location = 0) out vec4 outColor;

void main()
{
    float sqrDist = 2.0 * length(uvs - vec2(0.5, 0.5));

    if(sqrDist > 1.0) {
        discard;
    }
    else {
        outColor = vec4(color.rgb, color.a * smoothstep(0.0, 1.0, sqrt(1.0 - sqrDist)));
    }
}