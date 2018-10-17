
uniform sampler2D tex;
uniform vec3 color;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;
uniform vec2 mirror_center;
uniform mat2 mirror_rotation;
uniform vec2 mirror_size;

in vec3 nor;
in vec3 pos;
in vec2 uv;

layout (location = 0) out vec4 out_color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;
layout (location = 3) out vec4 surface_parameters;

void main()
{
    vec2 pos_mirror = mirror_rotation * (pos.xz - mirror_center);
    vec2 coords = (pos_mirror + 0.5 * mirror_size) / mirror_size;

    float blend = 0.1;
    out_color = vec4(blend * texture(tex, coords).rgb + (1.0 - blend) * color , 1.0);
    position = vec4(pos, 1.0);
    normal = vec4(nor, 1.0);
    surface_parameters = vec4(diffuse_intensity, specular_intensity, specular_power, 0.0);
}