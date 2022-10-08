uniform vec3 offset;
uniform mat4 viewProjection;
uniform float time;

uniform float wavelengths[4];
uniform float amplitudes[4];
uniform float steepnesses[4];
uniform float speeds[4];
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
        float wavelength = wavelengths[i];
        float amplitude = amplitudes[i];

        if(wavelength > 0.001 && amplitude > 0.001) {

            vec2 dir = directions[i];
            float speed = speeds[i];
            float steepness = steepnesses[i];
            
            float frequency = 2.0 * pi / wavelength;//sqrt(g * wavelength / (2.0 * pi)) * tanh(2.0 * pi * waterDepth / wavelength);
            float theta = dot(dir, pos.xz);
            float a = theta * frequency + time * speed;
            float sin_a = sin(a);
            float cos_a = cos(a);
            
            pos.y += amplitude * sin_a;
            pos.x += steepness * amplitude * dir.x * cos_a;
            pos.z += steepness * amplitude * dir.y * cos_a;
            
            nor.y -= steepness * frequency * amplitude * sin_a;
            nor.x -= dir.x * frequency * amplitude * cos_a;
            nor.z -= dir.y * frequency * amplitude * cos_a;
        }
    }
    
    gl_Position = viewProjection * vec4(pos, 1.);
    uvs = pos.xz;
}
