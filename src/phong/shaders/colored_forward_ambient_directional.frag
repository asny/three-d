
uniform vec4 surfaceColor;
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;

uniform vec3 ambientColor;

layout (std140) uniform DirectionalLightUniform
{
    DirectionalLight light;
};

in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    vec3 directional_color = calculate_directional_light(light, surfaceColor.rgb, pos, normal,
        diffuse_intensity, specular_intensity, specular_power);
    outColor = vec4(ambientColor * surfaceColor.rgb + directional_color, surfaceColor.a);
}