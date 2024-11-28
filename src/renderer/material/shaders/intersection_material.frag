
uniform vec3 eye;
uniform float minDistance;
uniform float maxDistance;
uniform uint geometryId;

in vec3 pos;
flat in int instance_id;

layout (location = 0) out vec4 outColor;

void main()
{
    float dist = (distance(pos, eye) - minDistance) / (maxDistance - minDistance);
    outColor = vec4(dist, uintBitsToFloat(geometryId), intBitsToFloat(instance_id), 1.0);
}