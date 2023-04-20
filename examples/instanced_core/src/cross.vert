in vec3 position;
in vec4 color;
in mat4 instance;
uniform mat4 viewProjection;
out vec4 v_color;

void main() {
    gl_Position = viewProjection * instance * vec4(position, 1.0);

    v_color = color;
}
