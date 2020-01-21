
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

in vec3 center;
in float theta;

in vec3 position;
in vec2 uv_coordinate;

out vec2 uv;
out float t;

void main()
{
    uv = uv_coordinate;
    vec3 dir = normalize(vec3(camera.view[0][2], 0.0, camera.view[2][2]));
    float c = dot(dir, vec3(0.0, 0.0, 1.0));
    float s = cross(dir, vec3(0.0, 0.0, 1.0)).y;
    mat3 rot = mat3(c, 0.0, s,
                0.0,  1.0, 0.0,
                -s,  0.0,  c);
    float angle = mod((s > 0.0 ? acos(c) : 2.0 * 3.1415926 - acos(c)) + theta, 3.1415926);
    t = angle / (2.0 * 3.1415926);
    gl_Position = camera.viewProjection * vec4(rot * position.xyz + center, 1.);
}
