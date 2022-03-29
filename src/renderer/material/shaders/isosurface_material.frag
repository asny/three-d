
uniform vec3 camera_position;
uniform vec4 surface_color;
uniform float metallic;
uniform float roughness;
uniform sampler3D tex;
uniform vec3 size;
uniform float threshold;

in vec3 pos;

layout (location = 0) out vec4 outColor;

vec3 estimate_normal(vec3 uvw) {
    float h = 0.01;
    float x = texture(tex, uvw + vec3(h, 0.0, 0.0)).r - texture(tex, uvw - vec3(h, 0.0, 0.0)).r;
    float y = texture(tex, uvw + vec3(0.0, h, 0.0)).r - texture(tex, uvw - vec3(0.0, h, 0.0)).r;
    float z = texture(tex, uvw + vec3(0.0, 0.0, h)).r - texture(tex, uvw - vec3(0.0, 0.0, h)).r;
    return -normalize(vec3(x, y, z) / (2.0 * h));
}

void main() {
    int steps = 200;
    float step_size = length(size) / float(steps);
    vec3 step = step_size * normalize(pos - camera_position);
    vec3 p = pos;
    for(int i = 0; i < 200; i++) {
        if(i == steps-1 || p.x < -0.501*size.x || p.y < -0.501*size.y || p.z < -0.501*size.z || p.x > 0.501*size.x || p.y > 0.501*size.y || p.z > 0.501*size.z) {
            outColor = vec4(0.0, 0.0, 0.0, 0.0);
            break;
        }
        vec3 uvw = (p / size) + 0.5;
        float value = texture(tex, uvw).r;
        if(value >= threshold) {
            vec3 normal = estimate_normal(uvw);
            outColor.rgb = calculate_lighting(surface_color.rgb, p, normal, metallic, roughness, 1.0);
            outColor.rgb = reinhard_tone_mapping(outColor.rgb);
            outColor.rgb = srgb_from_rgb(outColor.rgb);
            outColor.a = surface_color.a;
            break;
        }
        p += step;
    }
}