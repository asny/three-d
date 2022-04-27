
uniform mat4 viewProjection;
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
#ifdef USE_INSTANCE_TEXTURE_TRANSFORMATION
in vec3 tex_transform_row1;
in vec3 tex_transform_row2;
#endif
uniform mat3 textureTransform;
in vec2 uv_coordinates;
out vec2 uvs;
#endif

#ifdef USE_COLORS 
in vec4 color;
#ifdef USE_INSTANCE_COLORS
in vec4 instance_color;
#endif
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
    gl_Position = viewProjection * worldPosition;

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
    mat3 texTransform = textureTransform;
#ifdef USE_INSTANCE_TEXTURE_TRANSFORMATION
    mat3 instancedTexTransform;
    instancedTexTransform[0] = vec3(tex_transform_row1.x, tex_transform_row2.x, 0.0);
    instancedTexTransform[1] = vec3(tex_transform_row1.y, tex_transform_row2.y, 0.0);
    instancedTexTransform[2] = vec3(tex_transform_row1.z, tex_transform_row2.z, 1.0);
    texTransform *= instancedTexTransform;
#endif
    uvs = (texTransform * vec3(uv_coordinates, 1.0)).xy;
#endif

#ifdef USE_COLORS 
    col = color / 255.0;
#ifdef USE_INSTANCE_COLORS
    col *= instance_color / 255.0;
#endif
#endif
}