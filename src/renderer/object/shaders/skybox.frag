uniform samplerCube texture0;
uniform int isHDR;

in vec3 coords;

layout (location = 0) out vec4 outColor;

void main() {
    outColor = vec4(texture(texture0, coords).rgb, 1.0);
    if(isHDR > 0.5) {
        outColor.rgb = rgb_from_srgb(outColor.rgb);
        outColor.rgb = reinhard_tone_mapping(outColor.rgb);
        outColor.rgb = srgb_from_rgb(outColor.rgb);
    }
}
