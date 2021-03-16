
uniform float diffuse_intensity;
uniform float specular_intensity;
uniform float specular_power;
uniform sampler2D tex;

in vec3 pos;
in vec3 nor;
in vec2 uvs;

vec4 get_surface_color()
{
    return texture(tex, vec2(uvs.x, 1.0 - uvs.y));
}

Surface get_surface()
{
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);
    return Surface(pos, normal, get_surface_color(), diffuse_intensity, specular_intensity, specular_power);
}