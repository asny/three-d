

uniform samplerCube environmentMap;

in vec3 pos;

layout (location = 0) out vec4 outColor;

void main()
{		
	// The world vector acts as the normal of a tangent surface
    // from the origin, aligned to WorldPos. Given this normal, calculate all
    // incoming radiance of the environment. The result of this radiance
    // is the radiance of light coming from -Normal direction, which is what
    // we use in the PBR shader to sample irradiance.
    vec3 N = normalize(pos);

    vec3 irradiance = vec3(0.0);   
    
    // tangent space calculation from origin point
    vec3 up    = vec3(0.0, 1.0, 0.0);
    vec3 right = normalize(cross(up, N));
    up         = normalize(cross(N, right));
       
    float sampleDelta = 0.005 * PI;
    float nrSamples = 0.0;
    for(float phi = 0.0; phi < 2.0 * PI; phi += sampleDelta)
    {
        float sin_phi = sin(phi);
        float cos_phi = cos(phi);
        for(float theta = 0.0; theta < 0.5 * PI; theta += sampleDelta)
        {
            float sin_theta = sin(theta);
            float cos_theta = cos(theta);
            // spherical to cartesian (in tangent space)
            vec3 tangentSample = vec3(sin_theta * cos_phi,  sin_theta * sin_phi, cos_theta);
            // tangent space to world
            vec3 sampleVec = tangentSample.x * right + tangentSample.y * up + tangentSample.z * N; 

            irradiance += texture(environmentMap, sampleVec).rgb * cos_theta * sin_theta;
            nrSamples++;
        }
    }
    irradiance = PI * irradiance * (1.0 / float(nrSamples));
    
    outColor = vec4(irradiance, 1.0);
}