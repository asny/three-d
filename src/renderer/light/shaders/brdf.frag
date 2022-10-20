in vec2 uvs;

out vec2 FragColor;

vec2 IntegrateBRDF(float NdotV, float roughness)
{
    vec3 V;
    V.x = sqrt(1.0 - NdotV*NdotV);
    V.y = 0.0;
    V.z = NdotV;

    float A = 0.0;
    float B = 0.0; 

    vec3 N = vec3(0.0, 0.0, 1.0);
    
    const uint SAMPLE_COUNT = 1024u;
    for(uint i = 0u; i < SAMPLE_COUNT; ++i)
    {
        // generates a sample vector that's biased towards the
        // preferred alignment direction (importance sampling).
        vec2 Xi = Hammersley(i, SAMPLE_COUNT);
        vec3 H = ImportanceSampleGGX(Xi, N, roughness);
        vec3 L = normalize(2.0 * dot(V, H) * H - V);

        float NdL = max(L.z, 0.0);

        if(NdL > 0.0)
        {
            float NdH = max(H.z, 0.0);
            float VdH = max(dot(V, H), 0.0);
            float NdV = max(dot(N, V), 0.0);

            float G = G_schlick(roughness, NdV, NdL);
            float G_Vis = (G * VdH) / (NdH * NdV);
            float Fc = pow(1.0 - VdH, 5.0);

            A += (1.0 - Fc) * G_Vis;
            B += Fc * G_Vis;
        }
    }
    A /= float(SAMPLE_COUNT);
    B /= float(SAMPLE_COUNT);
    return vec2(A, B);
}

void main() 
{
    vec2 integratedBRDF = IntegrateBRDF(uvs.x, uvs.y);
    FragColor = integratedBRDF;
}