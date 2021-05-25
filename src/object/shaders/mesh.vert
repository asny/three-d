{} // Shared

layout (std140) uniform Camera
{{
    mat4 viewProjection;
    mat4 view;
    mat4 projection;
    vec3 position;
    float padding;
}} camera;

uniform mat4 modelMatrix;
in vec3 position;

{} // Instancing
{} // Positions out
{} // Normals in/out
{} // UV coordinates in/out
{} // Colors in/out

void main()
{{
    mat4 local2World = modelMatrix;
    {} // Instancing
    vec4 worldPosition = local2World * vec4(position, 1.);
    gl_Position = camera.viewProjection * worldPosition;
    {} // Position
    {} // Normal
    {} // UV coordinates
    {} // Colors
}}