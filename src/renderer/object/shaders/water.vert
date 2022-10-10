uniform vec3 offset;
uniform mat4 viewProjection;
uniform float time;

uniform vec4 waveParameters[4];
uniform vec2 directions[4];

const int noWaves = 4;
const float pi = 3.14159;

in vec3 position;

out vec2 uvs;
out vec3 nor;
out vec3 pos;

void main()
{
    pos = position + offset;
    nor = vec3(0., 1., 0.);
    
    // Offset position
    for (int i = 0; i < noWaves; ++i)
    {
        vec4 parameters = waveParameters[i];
        float wavelength = parameters.x;
        float amplitude = parameters.y;

        if(wavelength > 0.001 && amplitude > 0.001) {

            vec2 dir = directions[i];
            float steepness = parameters.z;
            float speed = parameters.w;
            
            float frequency = 2.0 * pi / wavelength;//sqrt(g * wavelength / (2.0 * pi)) * tanh(2.0 * pi * waterDepth / wavelength);
            float theta = dot(dir, pos.xz);
            float a = theta * frequency + time * speed;
            float sin_a = sin(a);
            float cos_a = cos(a);
            vec2 b = amplitude * cos_a * dir;
            float c = amplitude * sin_a;
            
            pos.y += c;
            pos.x += steepness * b.x;
            pos.z += steepness * b.y;
            
            nor.y -= steepness * frequency * c;
            nor.x -= frequency * b.x;
            nor.z -= frequency * b.y;
        }
    }
    
    gl_Position = viewProjection * vec4(pos, 1.);
    uvs = pos.xz;
}
