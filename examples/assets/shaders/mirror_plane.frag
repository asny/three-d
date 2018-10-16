
uniform sampler2D tex;
uniform vec3 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;
uniform vec2 mirror_center_uv;

in vec3 nor;
in vec3 pos;
in vec2 uv;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;
layout (location = 3) out vec4 surface_parameters;

void main()
{
    float blend = 0.5;
    vec2 coords = mirror_center_uv - uv + vec2(0.5, 0.5);
    out_color = vec4(blend * texture(tex, coords).rgb + (1.0 - blend) * color , 1.0);
    position = vec4(pos, 1.0);
    normal = vec4(nor, 1.0);
    surface_parameters = vec4(diffuse_intensity, specular_intensity, specular_power, 0.0);
}