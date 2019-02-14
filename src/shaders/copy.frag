uniform sampler2D colorMap;
uniform sampler2D depthMap;

in vec2 uv;

layout (location = 0) out vec4 color;

void main()
{
    color = vec4(texture(colorMap, uv).xyz, 1.);
    gl_FragDepth = texture(depthMap, uv).r;
}
