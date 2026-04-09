
uniform float metallic;
uniform float roughness;
uniform vec3 cameraPosition;

uniform vec4 albedo;
#ifdef USE_ALBEDO_TEXTURE
uniform sampler2D albedoTexture;
uniform mat3 albedoTexTransform;
#endif

uniform vec4 emissive;
#ifdef USE_EMISSIVE_TEXTURE
uniform sampler2D emissiveTexture;
uniform mat3 emissiveTexTransform;
#endif

#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
uniform sampler2D metallicRoughnessTexture;
uniform mat3 metallicRoughnessTexTransform;
#endif

#ifdef USE_OCCLUSION_TEXTURE
uniform sampler2D occlusionTexture;
uniform mat3 occlusionTexTransform;
uniform float occlusionStrength;
#endif

#ifdef USE_NORMAL_TEXTURE
uniform sampler2D normalTexture;
uniform mat3 normalTexTransform;
uniform float normalScale;
#endif

#ifdef USE_HEIGHT_TEXTURE
uniform sampler2D heightTexture;
uniform mat3 heightTexTransform;
uniform float heightScale;
uniform int heightMaxLayers;  // Precomputed: base_layers * height_scale-based multiplier
uniform int heightRefinementIterations;
uniform float heightFadeDistStart;  // Distance where POM starts fading
uniform float heightFadeDistEnd;    // Distance where POM is fully off
#endif

in vec3 pos;
in vec3 nor;
in vec4 col;

layout (location = 0) out vec4 outColor;

#ifdef USE_HEIGHT_TEXTURE
// Parallax Occlusion Mapping with smooth distance fade
// Full quality POM close up, smoothly blends to flat at distance
vec2 parallaxOcclusionMapping(vec2 texCoords, vec3 viewDirTangent, float dist, out float pomStrength) {
    // Smooth fade based on quality setting
    pomStrength = 1.0 - smoothstep(heightFadeDistStart, heightFadeDistEnd, dist);

    // Early out for distant surfaces
    if (pomStrength < 0.001) {
        return texCoords;
    }

    // Extract transform components once
    mat2 txLinear = mat2(heightTexTransform[0].xy, heightTexTransform[1].xy);
    vec2 txOffset = heightTexTransform[2].xy;

    // Layer count scales with pomStrength - fewer layers at distance
    float fNumLayers = max(2.0, floor(float(heightMaxLayers) * pomStrength + 0.5));
    float invNumLayers = 1.0 / fNumLayers;

    // Shift per layer (divide by z for correct view-angle scaling)
    vec2 P = viewDirTangent.xy / max(viewDirTangent.z, 0.0001) * heightScale;
    vec2 deltaUV = P * invNumLayers;

    // Start centered: offset by P*0.5 so 50% gray = surface level (DO NOT CHANGE)
    vec2 currentUV = texCoords + P * 0.5;
    float currentLayerDepth = 0.0;
    vec2 sampleUV = txLinear * currentUV + txOffset;
    float currentDepth = 1.0 - textureLod(heightTexture, sampleUV, 0.0).r;

    // Linear search
    for (float layer = 0.0; layer < fNumLayers && currentLayerDepth < currentDepth; layer += 1.0) {
        currentUV -= deltaUV;
        currentLayerDepth += invNumLayers;
        sampleUV = txLinear * currentUV + txOffset;
        currentDepth = 1.0 - textureLod(heightTexture, sampleUV, 0.0).r;
    }

    // Get previous position for interpolation
    vec2 prevUV = currentUV + deltaUV;
    float prevLayerDepth = currentLayerDepth - invNumLayers;
    float prevDepth = 1.0 - textureLod(heightTexture, txLinear * prevUV + txOffset, 0.0).r;

    // Track differences for secant method
    float d0 = prevDepth - prevLayerDepth;
    float d1 = currentDepth - currentLayerDepth;

    // Secant refinement (scale iterations by pomStrength)
    int refinementIters = int(float(heightRefinementIterations) * pomStrength + 0.5);
    for (int i = 0; i < refinementIters; i++) {
        float denom = d0 - d1;
        float t = (abs(denom) > 0.0001) ? d0 / denom : 0.5;
        vec2 newUV = mix(prevUV, currentUV, t);
        float newLayerDepth = mix(prevLayerDepth, currentLayerDepth, t);
        float newDepth = 1.0 - textureLod(heightTexture, txLinear * newUV + txOffset, 0.0).r;
        float dNew = newDepth - newLayerDepth;

        if (dNew > 0.0) {
            prevUV = newUV;
            prevLayerDepth = newLayerDepth;
            d0 = dNew;
        } else {
            currentUV = newUV;
            currentLayerDepth = newLayerDepth;
            d1 = dNew;
        }
    }

    // Final interpolation (guard against division by zero)
    float denom = d0 - d1;
    float t = (abs(denom) > 0.0001) ? d0 / denom : 0.5;
    return mix(prevUV, currentUV, clamp(t, 0.0, 1.0));
}
#endif

void main()
{
    vec3 normal = normalize(gl_FrontFacing ? nor : -nor);

    // Build TBN matrix early (needed for parallax and/or normal mapping)
#if defined(USE_NORMAL_TEXTURE) || defined(USE_HEIGHT_TEXTURE)
    vec3 tangent = normalize(gl_FrontFacing ? tang : -tang);
    vec3 bitangent = normalize(gl_FrontFacing ? bitang : -bitang);
    mat3 tbn = mat3(tangent, bitangent, normal);
#endif

    // Calculate parallax-displaced UVs with smooth distance fade
    // Compiler will optimize-out this assignment if USE_HEIGHT_TEXTURE is false
    vec2 texCoords = uvs;
#ifdef USE_HEIGHT_TEXTURE
    vec3 toCamera = cameraPosition - pos;
    float distToCamera = length(toCamera);
    vec3 viewDir = toCamera / max(distToCamera, 0.0001);
    float pomStrength;
    vec3 viewDirTangent = normalize(transpose(tbn) * viewDir);
    vec2 pomUV = parallaxOcclusionMapping(uvs, viewDirTangent, distToCamera, pomStrength);
    // Blend between POM result and flat UVs based on distance
    texCoords = mix(uvs, pomUV, pomStrength);
#endif

    vec4 surface_color = albedo * col;
#ifdef USE_ALBEDO_TEXTURE
    vec4 c = texture(albedoTexture, (albedoTexTransform * vec3(texCoords, 1.0)).xy);
    #ifdef ALPHACUT
        if (c.a < acut) discard;
    #endif
    surface_color *= c;
#endif

    float metallic_factor = metallic;
    float roughness_factor = roughness;
#ifdef USE_METALLIC_ROUGHNESS_TEXTURE
    vec2 t = texture(metallicRoughnessTexture, (metallicRoughnessTexTransform * vec3(texCoords, 1.0)).xy).gb;
    roughness_factor *= t.x;
    metallic_factor *= t.y;
#endif

    float occlusion = 1.0;
#ifdef USE_OCCLUSION_TEXTURE
    occlusion = mix(1.0, texture(occlusionTexture, (occlusionTexTransform * vec3(texCoords, 1.0)).xy).r, occlusionStrength);
#endif

    // Normal mapping
#if defined(USE_NORMAL_TEXTURE)
    normal = tbn * ((2.0 * texture(normalTexture, (normalTexTransform * vec3(texCoords, 1.0)).xy).xyz - 1.0) * vec3(normalScale, normalScale, 1.0));
#endif

    vec3 total_emissive = emissive.rgb;
#ifdef USE_EMISSIVE_TEXTURE
    total_emissive *= texture(emissiveTexture, (emissiveTexTransform * vec3(texCoords, 1.0)).xy).rgb;
#endif

    outColor.rgb = total_emissive + calculate_lighting(cameraPosition, surface_color.rgb, pos, normal, metallic_factor, roughness_factor, occlusion);
    outColor.rgb = tone_mapping(outColor.rgb);
    outColor.rgb = color_mapping(outColor.rgb);
    outColor.a = surface_color.a;
}
