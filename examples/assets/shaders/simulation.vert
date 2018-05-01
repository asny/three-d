uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

in vec3 Position;
in float FaceId;

out vec2 coords;
out float originFaceId;
out vec3 origin;

void main()
{
    originFaceId = FaceId;
    origin = (modelMatrix * vec4( Position, 1.0 )).xyz;
    gl_Position = projectionMatrix * viewMatrix * vec4(Position, 1.0);
}
