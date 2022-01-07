vec3 srgb_from_rgb(vec3 rgb) {
    vec3 a = vec3(0.055, 0.055, 0.055);
    vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
    vec3 g = vec3(2.4, 2.4, 2.4);
    vec3 ginv = 1.0 / g;
    vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), rgb);
    vec3 lo = rgb * 12.92;
    vec3 hi = ap1 * pow(rgb, ginv) - a;
    return mix(lo, hi, select);
}

vec3 reinhard_tone_mapping(vec3 color) {
    return color / (color + vec3(1.0));
}

uniform sampler2D image;
in vec2 uv;
layout (location = 0) out vec4 outColor;
void main()
{
    outColor = texture(image, vec2(uv.x, 1.0 - uv.y));
    outColor.rgb = reinhard_tone_mapping(outColor.rgb);
    outColor.rgb = srgb_from_rgb(outColor.rgb);
}