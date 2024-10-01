uniform int id;
uniform vec3 eye;

flat in int instanceID;

in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{
    float dist = distance(pos, eye);
    outColor = vec4(dist, float(id), float(instanceID), 1.0);
}