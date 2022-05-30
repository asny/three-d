
uniform vec3 eye;
uniform float minDistance;
uniform float maxDistance;

in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    float dist = (distance(pos, eye) - minDistance) / (maxDistance - minDistance);
    outColor = vec4(dist, dist, dist, 1.0);
}