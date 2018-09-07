uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;
in vec3 root_position;

out vec3 pos;
out vec3 nor;
out vec2 uv;

const float width = 0.2f;
const float height = 0.8f;
const vec3 up_direction = vec3(0., 1., 0.);
const vec3 tangent_side = vec3(1.0, 0.0, 0.0);

float func(float x)
{
    x = 0.5 * x;
    return -0.5625f * x*x + 0.75f * x;
}

float dfunc(float x)
{
    x = 0.5 * x;
    return -1.125f*x + 0.75f;
}

vec3 compute_position(vec3 origin, vec3 top, float parameter)
{
    return origin + parameter * (top - origin) + func(parameter) * up_direction;
}

vec3 compute_normal(vec3 origin, vec3 top, float parameter)
{
    vec3 tangent_up = top - origin + dfunc(parameter) * up_direction;
    return normalize(cross(tangent_side, tangent_up));
}

void main()
{
    vec3 origin = root_position + vec3(width * position.x - 0.5 * width, 0.0, 0.0);
    vec3 top = root_position + vec3(0.0, height, 0.5 * height);
    pos = compute_position(origin, top, position.y);
    nor = compute_normal(origin, top, position.y);
    uv = position.xy;
    gl_Position = projectionMatrix * viewMatrix * vec4(pos, 1.0);
}
