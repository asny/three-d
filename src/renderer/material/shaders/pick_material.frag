
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform float minDistance;
uniform float maxDistance;

in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    float dist = (distance(pos, camera.position) - minDistance) / (maxDistance - minDistance);
    outColor = vec4(dist, dist, dist, 1.0);
}