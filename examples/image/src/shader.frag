uniform sampler2D image;
uniform float parameter;
uniform mat3 textureTransform;

in vec2 uvs;

layout (location = 0) out vec4 outColor;

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
    return color / (color + vec3(parameter));
}

void main()
{
    vec3 uvw = textureTransform * vec3(uvs, 1.0);
    outColor = texture(image, uvw.xy / uvw.z);
    outColor.rgb = reinhard_tone_mapping(outColor.rgb);
    outColor.rgb = srgb_from_rgb(outColor.rgb);
}