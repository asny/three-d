uniform int id;
uniform vec3 eye;

flat in int instanceID;

in vec3 pos;

layout (location = 0) out ivec4 outColor;

void main()
{
    float dist = distance(pos, eye);
    outColor = ivec4(floatBitsToInt(dist), id, instanceID, -1);
}