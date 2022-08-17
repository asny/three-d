uniform vec3 cameraPosition;
uniform vec4 surfaceColor;
uniform float metallic;
uniform float roughness;
uniform sampler3D tex;
uniform vec3 size;
uniform float threshold;
uniform vec3 h;

in vec3 pos;

layout (location = 0) out vec4 outColor;

vec3 estimate_normal(vec3 uvw) {
    float x = texture(tex, uvw + vec3(h.x, 0.0, 0.0)).r - texture(tex, uvw - vec3(h.x, 0.0, 0.0)).r;
    float y = texture(tex, uvw + vec3(0.0, h.y, 0.0)).r - texture(tex, uvw - vec3(0.0, h.y, 0.0)).r;
    float z = texture(tex, uvw + vec3(0.0, 0.0, h.z)).r - texture(tex, uvw - vec3(0.0, 0.0, h.z)).r;
    return -normalize(vec3(x, y, z) / (2.0 * h));
}

void main() {
    int steps = 200;
    vec3 rayDir = normalize(pos - cameraPosition);
    // Start the ray from the camera position by default
    const float minDistFromCamera = 0.2;
    vec3 rayPos = cameraPosition + minDistFromCamera * rayDir;
    float stepSize = length(size) / float(steps);
    vec3 step = rayDir * stepSize;
    for (int i = 0; i < 200; i++) {
        if (i == steps-1) {
            // Out of steps: transparent
            outColor = vec4(0.0, 0.0, 0.0, 0.0);
            break;
        }
        if (rayPos.x < -0.501*size.x || rayPos.y < -0.501*size.y || rayPos.z < -0.501*size.z ||
        rayPos.x > 0.501*size.x || rayPos.y > 0.501*size.y || rayPos.z > 0.501*size.z) {
            // Out of bounds
            if (i == 0) {
                // Use the contact point on the box as the starting point
                rayPos = pos;
            } else {
                // Debug the number of steps:
                //outColor = vec4(0.0, float(i)/float(steps), 0.0, 1.0);
                outColor = vec4(0.0, 0.0, 0.0, 0.0);
                break;
            }
        }
        vec3 uvw = (rayPos / size) + 0.5;
        float surfaceDensity = texture(tex, uvw).r - threshold;
        if (surfaceDensity >= 0.0) { // We hit the surface
            vec3 normal = estimate_normal(uvw);
            outColor.rgb = calculate_lighting(cameraPosition, surfaceColor.rgb, rayPos, normal, metallic, roughness, 1.0);
            outColor.rgb = reinhard_tone_mapping(outColor.rgb);
            outColor.rgb = srgb_from_rgb(outColor.rgb);
            outColor.a = surfaceColor.a;
            break;
        }
        rayPos += step;
    }
}