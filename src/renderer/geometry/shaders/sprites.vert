uniform mat4 viewProjection;
uniform vec3 eye;
uniform mat4 transformation;
uniform vec3 direction;

in vec3 center;

in vec3 position;
in vec2 uv_coordinate;

out vec2 uvs;

void main()
{
    uvs = uv_coordinate;

    vec3 z = normalize(eye - center);
    vec3 y = direction;
    vec3 x;
    if (dot(y, y) < 0.01) {
        vec3 t = vec3(0.0, 1.0, 0.0);
        if(dot(t, z) > 0.99) {
            t = vec3(1.0, 0.0, 0.0);
        }
        x = normalize(cross(t, z));
        y = normalize(cross(z, x));
    } else {
        x = normalize(cross(y, z));
    }

    mat4 instanced_transform = mat4(x, 0.0,
                y, 0.0,
                z, 0.0,
                center.x, center.y, center.z, 1.0);
    gl_Position = viewProjection * instanced_transform * transformation * vec4(position, 1.);
}
