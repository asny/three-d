layout (std140) uniform Camera
{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
} camera;

uniform mat4 modelMatrix;
in vec3 position;

#ifdef INSTANCED
in vec4 row1;
in vec4 row2;
in vec4 row3;
#endif

#ifdef USE_POSITIONS
out vec3 pos;
#endif

#ifdef USE_NORMALS 
uniform mat4 normalMatrix;
in vec3 normal;
out vec3 nor;
#endif

#ifdef USE_UVS 
in vec2 uv_coordinates;
out vec2 uvs;
#endif

#ifdef USE_COLORS 
in vec4 color;
out vec4 col;
#endif

void main()
{
    mat4 local2World = modelMatrix;
#ifdef INSTANCED
    mat4 transform;
    transform[0] = vec4(row1.x, row2.x, row3.x, 0.0);
    transform[1] = vec4(row1.y, row2.y, row3.y, 0.0);
    transform[2] = vec4(row1.z, row2.z, row3.z, 0.0);
    transform[3] = vec4(row1.w, row2.w, row3.w, 1.0);
    local2World *= transform;
#endif
    vec4 worldPosition = local2World * vec4(position, 1.);
    gl_Position = camera.viewProjection * worldPosition;

#ifdef USE_POSITIONS
    pos = worldPosition.xyz;
#endif

#ifdef USE_NORMALS 
    nor = mat3(normalMatrix) * normal;
#endif

#ifdef USE_UVS 
    uvs = uv_coordinates;
#endif

#ifdef USE_COLORS 
    col = vec4(rgb_from_srgb(color.rgb/255.0), color.a/255.0);
#endif
}