
uniform mat4 viewProjection;
uniform mat4 modelMatrix;

in vec3 position;
in vec3 barycentric;

out vec3 pos;
out vec3 bary;

void main() {
    vec4 worldPos = modelMatrix * vec4(position, 1.);
    pos = worldPos.xyz;
    bary = barycentric;
    gl_Position = viewProjection * worldPos;
}