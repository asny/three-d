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
    float depth = texture(depthMap, vec3(uv,0)).r;
    if(depth > 0.99999)
    {
        discard;
    }
   	vec4 c = texture(gbuffer, vec3(uv, 0));
    color = vec4(c.rgb * ambientLight.base.color * ambientLight.base.intensity, 1.0);
}
