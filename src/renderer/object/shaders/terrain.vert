uniform mat4 viewProjectionMatrix;

in vec3 position;

out vec3 pos;
out vec2 uvs;

#ifdef USE_NORMALS

in vec3 normal;

out vec3 nor;
out vec3 tang;
out vec3 bitang;
#endif

void main()
{
    vec4 worldPos = vec4(position, 1.);
    pos = worldPos.xyz;
    uvs = worldPos.xz;
#ifdef USE_NORMALS
    nor = normalize(normal);
    tang = cross(vec3(1.0, 0.0, 0.0), nor);
    bitang = cross(nor, tang);
#endif
    gl_Position = viewProjectionMatrix * worldPos;
}
