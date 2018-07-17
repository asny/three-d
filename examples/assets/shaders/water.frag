uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;
uniform samplerCube environmentMap;
uniform vec3 eyePosition;
/*uniform float time;
uniform vec4 ringCenterAndTime[32];
uniform int noEffects;
uniform float ringEffectTime;*/

uniform sampler2D positionMap;
uniform sampler2D colorMap;
//uniform sampler2D maskSampler;
uniform vec2 screenSize;

in vec3 pos;
in vec3 nor;
in vec2 uv;

layout (location = 0) out vec4 color;

const float Eta = 1. / 1.5; // Ratio of indices of refraction
const float FresnelPower = 5.0;
const float F = ((1.0-Eta) * (1.0-Eta)) / ((1.0+Eta) * (1.0+Eta));

vec3 reflect_color(vec3 incidentDir, vec3 normal)
{
    vec3 reflectDir = normalize(reflect(incidentDir, normal));
    vec3 stepDir = 0.5 * reflectDir;
    vec3 p_w = pos;
    for (int i = 0; i < 8; i++)
    {
        p_w += stepDir;
        vec4 p_s = projectionMatrix * viewMatrix * vec4(p_w, 1.);
        p_s /= p_s.w;
        vec2 uv = 0.5 + 0.5 * p_s.xy;
        if(uv.x < 0. || uv.x > 1. || uv.y < 0. || uv.y > 1.)
            break;
        float dist = distance(eyePosition, p_w);
        float depth = distance(eyePosition, texture(positionMap, uv).xyz);
        if(depth < dist)
        {
            return texture(colorMap, uv).xyz;
        }
    }
    return texture(environmentMap, reflectDir).xyz;
}

vec3 fog(vec3 col, vec3 p1, vec3 p2)
{
    const float fogDensity = 0.08;
    const vec3 fogColor = vec3(0.8, 0.8, 0.8);
    const float noFogHeight = 6.;
    
    float a = p1.y;
    float b = p2.y;
    float t1 = clamp(-b/(a-b), 0., 1.);
    float t2 = clamp((noFogHeight-b)/(a-b), 0., 1.);
    
    float dist = min(distance(p1, p2), 100.) * abs(t1 - t2);
    
    float x = dist * fogDensity;
    float factor = 1. - 1. / exp(x * x);
    
    return mix(col, fogColor, factor);
}

vec3 water(vec3 col, vec3 p1, vec3 p2)
{
    const vec3 scattering = vec3(0.2, 0.4, 0.2); // Scattering coefficient (due to particles in the water)
    const vec3 absorption = vec3(0.4, 0.955, 0.99); // Absorption coefficient
    const vec3 c = scattering * absorption;
    const vec3 equilibriumColorAtInfinity = vec3(0., 0.1, 0.14); // Water color at "infinity"
    
    float dist = min(distance(p1, p2), 100.);
    vec3 colorChange = vec3(clamp( pow(c.r, dist), 0., 1.), clamp( pow(c.g, dist), 0., 1.), clamp( pow(c.b, dist), 0., 1.));
    return colorChange * col + (1 - colorChange) * equilibriumColorAtInfinity;
}

vec3 get_normal()
{
    // Rings
    vec3 ring = vec3(0., 0., 0.);
    /*for (int i = 0; i < noEffects; i++)
    {
        vec3 center = ringCenterAndTime[i].xyz;
        float startTime = ringCenterAndTime[i].w;
        float dist = distance(pos, center);
        float timeSinceStart = time - startTime;
        float spread = 0.25 * timeSinceStart;
        if(startTime >= 0. && dist < spread)
        {
            float fadeFactor = 1. - smoothstep(0., ringEffectTime, timeSinceStart);
            float ringFactor = 0.4 * smoothstep(0., spread, dist) * sin(100.*(spread - dist));
            ring += fadeFactor * ringFactor * normalize(pos - center);
        }
    }*/
    return normalize(nor + ring);
}

void main()
{
    vec2 screen_uv = gl_FragCoord.xy/screenSize;
    vec3 backgroundPos = texture(positionMap, screen_uv).xyz;
    
    // Do not draw when something is in front of the water.
    if(dot(eyePosition - pos, eyePosition - backgroundPos) < 0.0)
    {
        discard;
    }
    
    vec3 normal = get_normal();
    vec3 incidentDir = normalize(pos - eyePosition);
    screen_uv -= 0.05 * normal.xz; // Shift the water bottom/sky.
    backgroundPos = texture(positionMap, screen_uv).xyz;
    vec3 col = texture(colorMap, screen_uv).xyz;
    
    bool underWater = dot(normal, incidentDir) > 0.1 || dot(normal, incidentDir) > -0.1 && eyePosition.y < 0.0;
    
    if(underWater)
    {
        col = fog(col, pos, backgroundPos);
        col = water(col, eyePosition, pos);
        color = vec4(col, 1.);
        return;
    }
    
    // Compute cosine to the incident angle
    float cosAngle = dot(normal, -incidentDir);
    
    // Compute fresnel approximation
    float fresnel = mix(F, 1.f, pow(1. - max(cosAngle, 0.), FresnelPower));
    
    // Reflection
    vec3 reflectColor = mix(reflect_color(incidentDir, normal), vec3(1., 1., 1.), 0.5);
    
    // Refraction
    vec3 refractColor = water(col, pos, backgroundPos);
    
    // Mix refraction and reflection
    col = mix(refractColor, reflectColor, fresnel);
    
    // Foam
    /*float waterDepth = pos.y-backgroundPos.y;
    const float minFoamDepth = 0.;
    const float peakFoamDepth = 0.02;
    const float maxFoamDepth = 0.1;
    if(waterDepth > minFoamDepth && waterDepth < maxFoamDepth)
    {
        float foam;
        if(waterDepth < peakFoamDepth)
        {
            foam = smoothstep(minFoamDepth, peakFoamDepth, waterDepth);
        }
        else {
            foam = 1. - ((waterDepth - peakFoamDepth) / (maxFoamDepth - peakFoamDepth));
        }
        
        // Sample the mask
        vec2 scaledUV = uv * 20.;
        float edgePatternScroll = 0.2;
        float channelA = texture(maskSampler, scaledUV - vec2(edgePatternScroll, cos(uv.x))).r;
        float channelB = texture(maskSampler, scaledUV * 0.5 + vec2(sin(uv.y), edgePatternScroll)).b;
        
        // Modify it to our liking
        float mask = (channelA + channelB) * 0.95;
        mask = pow(mask, 2);
        mask = clamp(mask, 0, 1);
        
        foam = clamp(foam - mask, 0, 0.5);
        col = mix(col, vec3(0.8,0.8,0.8), foam);
    }*/
    
    // Fog
    col = fog(col, eyePosition, pos);
    
    color = vec4(col, 1.);
}
