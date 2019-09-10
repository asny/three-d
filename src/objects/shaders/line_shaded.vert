
layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform float tube_radius;

in vec3 direction;
in vec3 translation;

in vec3 position;

out vec3 pos;
out vec3 nor;

mat3 rotationMatrix(vec3 source_dir, vec3 target_dir)
{
    vec3 axis = normalize(cross(source_dir, target_dir));
    float c = dot(source_dir, target_dir);
    float s = sqrt(1.0 - c*c);
    float oc = 1.0 - c;

    return mat3(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,
                oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,
                oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c);
}

void main()
{
    mat3 l2w = transpose(rotationMatrix(vec3(1.0, 0.0, 0.0), normalize(direction)));
    pos = l2w * (position * vec3(length(direction), tube_radius, tube_radius)) + translation;
    mat3 normalMatrix = transpose(inverse(l2w));
    nor = normalize(normalMatrix * vec3(0.0, position.y, position.z));
    gl_Position = camera.viewProjection * vec4(pos, 1.0);
}