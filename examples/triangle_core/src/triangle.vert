in vec3 position;
in vec4 color;
uniform mat4 model;
uniform mat4 projection;
out vec4 v_color;

void main() {
    vec4 worldPosition = model * vec4(position, 1.0);
    worldPosition /= worldPosition.w;
    gl_Position = projection * worldPosition;

    v_color = color;
}
