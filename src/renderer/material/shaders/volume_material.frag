layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform sampler3D tex;
uniform float max_distance;
in vec3 pos;

layout (location = 0) out vec4 outColor;

void main() {
    int steps = 200;
    float step_size = max_distance / float(steps);
    vec3 step = step_size * normalize(pos - camera.position);
    vec3 p = pos;
    for(int i = 0; i < 200; i++) {
        if(i == steps-1 || p.x < -1.0 || p.y < -1.0 || p.z < -1.0 || p.x > 1.0 || p.y > 1.0 || p.z > 1.0) {
            outColor = vec4(0.0, 0.0, 0.0, 0.0);
            break;
        }
        float color = texture(tex, 0.5*(p + 1.0)).r;
        if(color >= 0.9) {
            vec3 normal = vec3(0.0, 1.0, 0.0);

            vec4 surface_color = vec4(1.0, 0.5, 0.5, 1.0);
            float metallic_factor = 0.5;
            float roughness_factor = 0.6;
            float occlusion = 1.0;
            outColor.rgb = calculate_lighting(surface_color.rgb, p, normal, metallic_factor, roughness_factor, occlusion);
            outColor.rgb = reinhard_tone_mapping(outColor.rgb);
            outColor.rgb = srgb_from_rgb(outColor.rgb);
            outColor.a = surface_color.a;
            break;
        }
        p += step;
    }
}