uniform sampler2DArray gbuffer;
uniform sampler2DArray depthMap;

layout (location = 0) out vec4 color;

in vec2 uv;

struct BaseLight
{
    vec3 color;
    float intensity;
};

struct AmbientLight
{
    BaseLight base;
};

uniform AmbientLight ambientLight;

void main()
{
   	vec4 c = texture(gbuffer, vec3(uv, 0));
    bool is_far_away = texture(depthMap, vec3(uv,0)).r > 0.99999;
    color = vec4(c.rgb * ambientLight.base.color * (is_far_away? 1.0 : ambientLight.base.intensity), 1.0);
}
