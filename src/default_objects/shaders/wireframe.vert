
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 position0;
in vec3 position1;

in vec3 position;

out vec3 pos;
out vec3 nor;

mat4 rotationMatrix(vec3 axis, float c)
{
    axis = normalize(axis);
    float s = sqrt(1.0 - c*c);
    float oc = 1.0 - c;

    return mat4(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
                oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
                oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
                0.0,                                0.0,                                0.0,                                1.0);
}

void main()
{
    pos = position;
    pos.x *= distance(position1, position0);
    pos.y *= 0.02;
    pos.z *= 0.02;

    vec3 dir = normalize(position1 - position0);
    float cos_angle = dot(vec3(1.0, 0.0, 0.0), dir);
    vec3 axis;
    if(cos_angle > 0.99 || cos_angle < -0.99)
    {
        axis = -cross(vec3(0.0, 1.0, 0.0), dir);
    }
    else
    {
        axis = -cross(vec3(1.0, 0.0, 0.0), dir);
    }

    mat4 rot = rotationMatrix(axis, cos_angle);

    pos = (rot * vec4(pos, 1.0)).xyz;

    pos += position0;
    nor = vec3(0.0, 1.0, 0.0);

    gl_Position = projectionMatrix * viewMatrix * vec4(pos, 1.0);
}