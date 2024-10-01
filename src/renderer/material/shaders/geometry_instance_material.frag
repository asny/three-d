uniform float id;
uniform vec3 eye;

in vec3 pos;
in vec4 col;

layout (location = 0) out vec4 outColor;

void main()
{
    float instanceID = -col.x;

    float dist = distance(pos, eye);
    outColor = vec4(dist, id, instanceID, 1.0);
}