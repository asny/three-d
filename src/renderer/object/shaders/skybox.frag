uniform samplerCube texture0;

in vec3 coords;

layout (location = 0) out vec4 outColor;

void main() {
    outColor = vec4(texture(texture0, coords).rgb, 1.0);
}
