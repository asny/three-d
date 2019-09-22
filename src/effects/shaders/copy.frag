uniform sampler2DArray colorMap;
uniform sampler2DArray depthMap;

in vec2 uv;

layout (location = 0) out vec4 color;

void main()
{
    color = vec4(texture(colorMap, vec3(uv, 0)).xyz, 1.);
    gl_FragDepth = texture(depthMap, vec3(uv, 0)).r;
}
