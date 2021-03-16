
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;
uniform vec4 surfaceColor;

in vec3 pos;
in vec3 nor;

vec4 get_surface_color()
{
    return surfaceColor;
}

Surface get_surface()
{
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    return Surface(pos, normal, get_surface_color(), diffuse_intensity, specular_intensity, specular_power);
}