
uniform vec3 eye;
uniform float z_near;
uniform float z_far;
uniform float geometry_id;

in vec3 pos;
layout (location = 0) out vec4 outColor;

void main()
{ 
    outColor.r = geometry_id;
    outColor.g = (distance(pos, eye) - z_near) / (z_far - z_near);  
}
