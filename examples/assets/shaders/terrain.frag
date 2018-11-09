uniform sampler2D groundTexture;
uniform sampler2D lakeTexture;
uniform sampler2D noiseTexture;

in vec2 uv;
in vec3 nor;
in vec3 pos;

layout (location = 0) out vec4 color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;
layout (location = 3) out vec4 surface_parameters;

const vec3 absorptionColor = vec3(0.,0.3,0.3);

void main()
{
    position = vec4(pos, 1.0);
    
    // Sand color
    float sand_factor = clamp(5. * abs(pos.y), 0., 1.);
    float noise = 0.75 + 0.25 * texture(noiseTexture, pos.xz).x;
    vec3 sand_color = noise * (0.75 + 0.25 * sand_factor) * vec3(1., 1., 0.8);
    vec3 blend_color;
    if(pos.y > 0.)
    {
        blend_color = texture(groundTexture, uv).xyz;
    }
    else
    {
        blend_color = texture(lakeTexture, 3.*uv).xyz;
    }
    color = vec4(mix(sand_color, blend_color, sand_factor), 1.);
    normal = vec4(nor, 1.0);
    surface_parameters = vec4(0.5, 0.0, 0.0, 0.0);
}
