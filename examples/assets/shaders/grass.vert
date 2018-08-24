uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position;
in vec2 uv_coordinate;
in vec3 root_position;

out vec3 pos;
out vec3 nor;
out vec2 uv;

const float half_width = 0.01f;
const vec3 up_direction = vec3(0., 1., 0.);

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

vec3 compute_normal(vec3 origin, vec3 corner, vec3 top, float parameter)
{
    vec3 tangent = top - origin + dfunc(parameter) * up_direction;
    return normalize(cross(corner - origin, tangent));
}

void main()
{
    gl_Position = vec4(position, 1.0);
    vec3 p3 = compute_position(root_position + vec3(0.2 * uv_coordinate.x - 0.05, 0.0, 0.0), root_position + vec3(0.0, 0.5, 0.4), position.y);
    vec4 p = projectionMatrix * viewMatrix * vec4(p3, 1.0);
    gl_Position = p;
    pos = p.xyz;
    nor = vec3(0.0, 1.0, 0.0);
    uv = uv_coordinate;
}
