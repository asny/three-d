
uniform vec4 color;
uniform float fade;

in vec2 uvs;
in vec4 col;

layout (location = 0) out vec4 outColor;

vec3 srgb_from_linear_srgb(vec3 rgb) {
    vec3 a = vec3(0.055, 0.055, 0.055);
    vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
    vec3 g = vec3(2.4, 2.4, 2.4);
    vec3 ginv = 1.0 / g;
    vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), rgb);
    vec3 lo = rgb * 12.92;
    vec3 hi = ap1 * pow(rgb, ginv) - a;
    return mix(lo, hi, select);
}

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

    outColor.rgb = srgb_from_linear_srgb(outColor.rgb);
}