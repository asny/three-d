
layout (location = 0) out vec4 outColor;

uniform float lineWidth;
uniform vec4 lineColor;

in vec3 bary;
in vec3 pos;

void main() {
    vec3 d = fwidth(bary);
    vec3 f = step(d * lineWidth, bary);
    float b = min(min(f.x, f.y), f.z);
    outColor = lineColor;
    outColor.a *= 1.-b;
    // Move the line a bit closer to the camera to avoid z-fighting when rendering the model as well
    gl_FragDepth = gl_FragCoord.z - 0.0001; 
}