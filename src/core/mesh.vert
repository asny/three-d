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

#ifdef USE_TANGENTS 
in vec4 tangent;
out vec3 tang;
out vec3 bitang;
#endif

#endif


#ifdef USE_UVS 
#ifdef INSTANCED
in vec4 subt;
#endif
uniform vec4 textureTransform;
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
    mat3 normalMat;
#ifdef INSTANCED
    mat4 transform;
    transform[0] = vec4(row1.x, row2.x, row3.x, 0.0);
    transform[1] = vec4(row1.y, row2.y, row3.y, 0.0);
    transform[2] = vec4(row1.z, row2.z, row3.z, 0.0);
    transform[3] = vec4(row1.w, row2.w, row3.w, 1.0);
    local2World *= transform;

#ifdef USE_NORMALS
    normalMat = mat3(transpose(inverse(local2World)));
#endif
#else
#ifdef USE_NORMALS
    normalMat = mat3(normalMatrix);
#endif
#endif

    vec4 worldPosition = local2World * vec4(position, 1.);
    gl_Position = camera.viewProjection * worldPosition;

#ifdef USE_POSITIONS
    pos = worldPosition.xyz;
#endif

#ifdef USE_NORMALS 
    nor = normalize(normalMat * normal);

#ifdef USE_TANGENTS 
    tang = normalize(normalMat * tangent.xyz);
    bitang = normalize(cross(nor, tang) * tangent.w);
#endif

#endif

#ifdef USE_UVS 
    uvs = uv_coordinates * textureTransform.zw + textureTransform.xy;
#ifdef INSTANCED
    uvs = uvs * subt.zw + subt.xy;
#endif
#endif

#ifdef USE_COLORS 
    col = color/255.0;
#endif
}