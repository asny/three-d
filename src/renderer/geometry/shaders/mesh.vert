
uniform mat4 viewProjection;
uniform mat4 modelMatrix;
in vec3 position;

#ifdef PARTICLES
in vec3 start_position;
in vec3 start_velocity;
uniform vec3 acceleration;
uniform float time;
#endif

#ifdef USE_INSTANCE_TRANSLATIONS
in vec3 instance_translation;
#endif

#ifdef USE_INSTANCE_TRANSFORMS
in vec4 row1;
in vec4 row2;
in vec4 row3;
#endif

out vec3 pos;

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
in vec2 uv_coordinates;
out vec2 uvs;
#endif

#ifdef USE_VERTEX_COLORS 
in vec4 color;
#endif
#ifdef USE_INSTANCE_COLORS
in vec4 instance_color;
#endif

out vec4 col;

void main()
{
    // *** POSITION ***
    mat4 local2World = modelMatrix;
    
#ifdef USE_INSTANCE_TRANSFORMS
    mat4 transform;
    transform[0] = vec4(row1.x, row2.x, row3.x, 0.0);
    transform[1] = vec4(row1.y, row2.y, row3.y, 0.0);
    transform[2] = vec4(row1.z, row2.z, row3.z, 0.0);
    transform[3] = vec4(row1.w, row2.w, row3.w, 1.0);
    local2World *= transform;
#endif

    vec4 worldPosition = local2World * vec4(position, 1.);
    worldPosition /= worldPosition.w;
#ifdef PARTICLES
    worldPosition.xyz += start_position + start_velocity * time + 0.5 * acceleration * time * time;
#endif
#ifdef USE_INSTANCE_TRANSLATIONS 
    worldPosition.xyz += instance_translation;
#endif
    gl_Position = viewProjection * worldPosition;

    pos = worldPosition.xyz;

    // *** NORMAL ***
#ifdef USE_NORMALS 
#ifdef USE_INSTANCE_TRANSFORMS
    mat3 normalMat = mat3(transpose(inverse(local2World)));
#else
    mat3 normalMat = mat3(normalMatrix);
#endif
    nor = normalize(normalMat * normal);

#ifdef USE_TANGENTS 
    tang = normalize(normalMat * tangent.xyz);
    bitang = normalize(cross(nor, tang) * tangent.w);
#endif

#endif

    // *** UV ***
#ifdef USE_UVS 
#ifdef USE_INSTANCE_TEXTURE_TRANSFORMATION
    mat3 texTransform;
    texTransform[0] = vec3(tex_transform_row1.x, tex_transform_row2.x, 0.0);
    texTransform[1] = vec3(tex_transform_row1.y, tex_transform_row2.y, 0.0);
    texTransform[2] = vec3(tex_transform_row1.z, tex_transform_row2.z, 1.0);
    uvs = (texTransform * vec3(uv_coordinates, 1.0)).xy;
#else
    uvs = uv_coordinates;
#endif
#endif

    // *** COLOR ***
    col = vec4(1.0);
#ifdef USE_VERTEX_COLORS 
    col *= color;
#endif
#ifdef USE_INSTANCE_COLORS
    col *= instance_color;
#endif
}