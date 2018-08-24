
in vec3 nor;
in vec3 pos;
in vec2 uv;

layout (location = 0) out vec4 color;
layout (location = 1) out vec4 position;
layout (location = 2) out vec4 normal;

void main()
{
    color = vec4(uv, 0.0, 1.0f);
    position = vec4(pos, 1.0);
    normal = vec4(nor, 1.0);
}
