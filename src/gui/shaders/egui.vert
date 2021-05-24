uniform vec2 u_screen_size;

in vec2 a_pos;
in vec2 a_tc;
in vec4 a_srgba;

out vec4 v_rgba;
out vec2 v_tc;

void main() {
    gl_Position = vec4(
        2.0 * a_pos.x / u_screen_size.x - 1.0,
        1.0 - 2.0 * a_pos.y / u_screen_size.y,
        0.0,
        1.0);
    // egui encodes vertex colors in gamma spaces, so we must decode the colors here:
    v_rgba.rgb = rgb_from_srgb(a_srgba.rgb/255.0);
    v_rgba.a = a_srgba.a/255.0;
    v_tc = a_tc;
}