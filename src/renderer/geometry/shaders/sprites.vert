
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform mat4 transformation;

in vec3 center;

in vec3 position;
in vec2 uv_coordinate;

out vec2 uvs;

void main()
{
    uvs = uv_coordinate;
    vec3 dir = normalize(vec3(camera.view[0][2], 0.0, camera.view[2][2]));
    float c = dot(dir, vec3(0.0, 0.0, 1.0));
    float s = cross(dir, vec3(0.0, 0.0, 1.0)).y;
    mat4 instanced_transform = mat4(c, 0.0, s, 0.0,
                0.0,  1.0, 0.0, 0.0,
                -s,  0.0,  c, 0.0,
                center.x, center.y, center.z, 1.0);
    gl_Position = camera.viewProjection * instanced_transform * transformation * vec4(position, 1.);
}
