
layout (location = 0) out vec4 outColor;

uniform float u_line_width = 0.5;

in vec3 bary;
in vec3 pos;

void main() {
    vec3 d = fwidth(bary);
    vec3 f = step(d * u_line_width, bary);
    float b = min(min(f.x, f.y), f.z);
    outColor = vec4(1.-b);
}