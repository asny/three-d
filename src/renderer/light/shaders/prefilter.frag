out vec4 FragColor;
in vec3 pos;

uniform samplerCube environmentMap;
uniform float roughness;
uniform float resolution; // resolution of source cubemap (per face)

void main()
{		
    vec3 N = normalize(pos);
    
    // make the simplyfying assumption that V equals R equals the normal 
    vec3 R = N;
    vec3 V = R;

    const uint SAMPLE_COUNT = 1024u;
    vec3 prefilteredColor = vec3(0.0);
    float totalWeight = 0.0;
    
    for(uint i = 0u; i < SAMPLE_COUNT; ++i)
    {
        // generates a sample vector that's biased towards the preferred alignment direction (importance sampling).
        vec2 Xi = Hammersley(i, SAMPLE_COUNT);
        vec3 H = ImportanceSampleGGX(Xi, N, roughness);
        vec3 L  = normalize(2.0 * dot(V, H) * H - V);

        float NdL = max(dot(N, L), 0.0);
        if(NdL > 0.0)
        {
            float NdH = max(dot(N, H), 0.0);
            float HdV = max(dot(H, V), 0.0);

            // sample from the environment's mip level based on roughness/pdf
            float D = calculate_D(roughness, NdH);
            float pdf = D * NdH / (4.0 * HdV) + 0.0001; 

            float saTexel  = PI / (6.0 * resolution * resolution);
            float saSample = 1.0 / (float(SAMPLE_COUNT) * pdf + 0.0001);

            float mipLevel = roughness == 0.0 ? 0.0 : 0.5 * log2(saSample / saTexel); 
            
            prefilteredColor += textureLod(environmentMap, L, mipLevel).rgb * NdL;
            totalWeight      += NdL;
        }
    }

    prefilteredColor = prefilteredColor / totalWeight;

    FragColor = vec4(prefilteredColor, 1.0);
}