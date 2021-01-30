
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
        float f = 1.0 - sqrDist*sqrDist;
        outColor = vec4(color.rgb, color.a * f);
    }
}