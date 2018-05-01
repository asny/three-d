
uniform sampler2D tex;
uniform vec3 cameraPosition;

in vec3 col;
in vec3 posWorld;

out vec4 fragmentColor;

void main()
{
    float c = texture(tex, posWorld.xy).r;
    vec3 V = normalize(cameraPosition - posWorld);
    float l = clamp(dot(V, vec3(0, 1, 0)), 0, 1);
    fragmentColor = vec4(col * c * l, 1.0f);
}
