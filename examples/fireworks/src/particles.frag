
uniform vec4 color;
uniform float fade;

in vec2 uvs;
in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    float sqrDist = 2.0 * length(uvs - vec2(0.5, 0.5));

    if(sqrDist > 1.0) {
        discard;
    }
    else {
        float f = 1.0 - sqrDist*sqrDist;
        outColor = vec4(col.rgb + color.rgb, fade * color.a * f);
    }
}