
uniform sampler2D equirectangularMap;
const vec2 invAtan = vec2(0.1591, 0.3183);

in vec3 pos;

layout (location = 0) out vec4 outColor;

vec2 sample_spherical_map(vec3 v)
{
    vec2 uv = vec2(atan(v.z, v.x), asin(v.y));
    uv *= invAtan;
    uv += 0.5;
    return vec2(uv.x, 1.0 - uv.y);
}

void main()
{		
    vec2 uv = sample_spherical_map(normalize(pos));
    outColor = vec4(texture(equirectangularMap, uv).rgb, 1.0);
}
