
uniform sampler2D tex;
uniform vec3 cameraPosition;

in vec3 posWorld;

layout (location = 0) out vec4 color;

void main()
{
    float c = texture(tex, posWorld.xy).r;
    vec3 V = normalize(cameraPosition - posWorld);
    float l = 0.5 + 0.5 * dot(V, vec3(0, 1, 0));
    color = vec4(c * l, c * l, c * l, 1.0f);
}
