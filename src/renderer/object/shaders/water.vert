uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
uniform float time;

uniform float minWavelength;
uniform float maxWavelength;
uniform float speed = 0.5;
const float wind_variation = 0.1;
const vec2 wind_direction = vec2(1.0, 0.0);

const int noWaves = 4;
const float pi = 3.14159;

in vec3 position;

out vec2 uvs;
out vec3 nor;
out vec3 pos;

void main()
{
    pos = (modelMatrix * vec4(position, 1.)).xyz;
    pos.y = 0.;
    nor = vec3(0., 1., 0.);
    
    float wavelength_var[noWaves];
    wavelength_var[0] = 0.146;
    wavelength_var[1] = 0.335;
    wavelength_var[2] = 0.64632;
    wavelength_var[3] = 0.73134;
    
    float direction_var[noWaves];
    direction_var[0] = 0.821;
    direction_var[1] = 0.4572;
    direction_var[2] = 0.014;
    direction_var[3] = 0.71;
    
    // Offset position
    for (int i = 0; i < noWaves; ++i)
    {
        float wavelength = mix(minWavelength, maxWavelength, wavelength_var[i]);
        
        float dir_angle = wind_variation * pi * (2.0 * direction_var[i] - 1.0);
        float cos_angle = cos(dir_angle);
        float sin_angle = sin(dir_angle);
        vec2 dir = normalize(vec2( cos_angle * wind_direction.x - sin_angle * wind_direction.y,
                                  sin_angle * wind_direction.x + cos_angle * wind_direction.y));
        
        float frequency = 2.0 * pi / wavelength;//sqrt(g * wavelength / (2.0 * pi)) * tanh(2.0 * pi * waterDepth / wavelength);
        float amplitude = wavelength / 100.0;
        float steepness = wavelength_var[i]/(frequency * amplitude * float(noWaves));
        float phase = speed * frequency;
        
        float theta = dot(dir, pos.xz);
        float a = theta * frequency + time * phase;
        float sin_a = sin(a);
        float cos_a = cos(a);
        
        pos.y += amplitude * sin_a;
        pos.x += steepness * amplitude * dir.x * cos_a;
        pos.z += steepness * amplitude * dir.y * cos_a;
        
        nor.y -= steepness * frequency * amplitude * sin_a;
        nor.x -= dir.x * frequency * amplitude * cos_a;
        nor.z -= dir.y * frequency * amplitude * cos_a;
    }
    
    gl_Position = projectionMatrix * viewMatrix * vec4(pos, 1.);
    uvs = pos.xz;
}
