
uniform sampler2D indexToPosition;
uniform sampler2D faceIdToIndices;
uniform sampler2D cellIdToFaceIds;
uniform sampler2D faceIdToCellIds;
uniform sampler2D vertexIdToData;

uniform vec3 cameraPosition;

uniform float textureSpacing;
uniform float density;

in float originFaceId;
in vec3 origin;

out vec4 fragmentColor;

vec2 indexToUv(float index) {
    float x = (index + 0.5) * textureSpacing;
    return vec2( mod(x, 1.0) , 0.5);//floor(x) * textureSpacing );
}

vec2 getCellIDs(float fid){
    float cellId1 = texture( faceIdToCellIds, indexToUv(2.0 * fid) ).r;
    float cellId2 = texture( faceIdToCellIds, indexToUv(2.0 * fid + 1.0)).r;
    return vec2(cellId1, cellId2);
}

vec4 getFaceIDs(float cid){
    float faceId1 = texture( cellIdToFaceIds, indexToUv(4.0 * cid) ).r;
    float faceId2 = texture( cellIdToFaceIds, indexToUv(4.0 * cid + 1.0) ).r;
    float faceId3 = texture( cellIdToFaceIds, indexToUv(4.0 * cid + 2.0) ).r;
    float faceId4 = texture( cellIdToFaceIds, indexToUv(4.0 * cid + 3.0) ).r;
    return vec4(faceId1, faceId2, faceId3, faceId4);
}

vec3 getIndices(float fid){
    float index1 = texture( faceIdToIndices, indexToUv(3.0 * fid) ).r;
    float index2 = texture( faceIdToIndices, indexToUv(3.0 * fid + 1.0) ).r;
    float index3 = texture( faceIdToIndices, indexToUv(3.0 * fid + 2.0) ).r;
    return vec3(index1, index2, index3);
}

vec3 getPosition(float index){
    return texture( indexToPosition, indexToUv(index) ).rgb;
}

float getValue(float fid, vec3 barycentricCoords){
    return 1.0;
    vec3 indices = getIndices(fid);
    float val1 = texture( vertexIdToData, indexToUv(indices.x) ).r;
    float val2 = texture( vertexIdToData, indexToUv(indices.y) ).r;
    float val3 = texture( vertexIdToData, indexToUv(indices.z) ).r;
    return barycentricCoords.x * val3 + barycentricCoords.y * val1 + barycentricCoords.z * val2;
}

float pluckerTest(vec3 o, vec3 d, vec3 p1, vec3 p2){
    vec3 p_r = d;
    vec3 q_r = cross(o, d);
    vec3 p_e = p2 - p1;
    vec3 q_e = cross(p1, p2);
    return dot(p_r, q_e) + dot(q_r, p_e);
}

bool hitFace(vec3 o, vec3 d, float faceId, out vec3 hitPos, out vec3 barycentricCoords){
    vec3 indices = getIndices(faceId);
    vec3 pos1 = getPosition(indices.x);
    vec3 pos2 = getPosition(indices.y);
    vec3 pos3 = getPosition(indices.z);
    float test1 = pluckerTest(o, d, pos1, pos2);
    float test2 = pluckerTest(o, d, pos2, pos3);
    float test3 = pluckerTest(o, d, pos3, pos1);
    bool hit = test1 > 0.0 && test2 > 0.0 && test3 > 0.0 || test1 < 0.0 && test2 < 0.0 && test3 < 0.0;
    if(hit) {
        float sumTest = test1 + test2 + test3;
        barycentricCoords = vec3(test1/sumTest, test2/sumTest, test3/sumTest);
        hitPos = pos3 * barycentricCoords.x + pos1 * barycentricCoords.y + pos2 * barycentricCoords.z;
    }
    return hit;
}

bool next(vec3 o, vec3 d, inout float cellId, inout float faceId, out vec3 hitPos, out vec3 barycentricCoords){
    vec4 faceIds = getFaceIDs(cellId);
    for(int i = 0; i < 4; i++) {
        float fid = faceIds[i];
        if(abs(fid - faceId) > 0.01) {
            bool hit = hitFace(o, d, fid, hitPos, barycentricCoords);
            if(hit) {
                faceId = fid;
                vec2 cellIds = getCellIDs(fid);
                cellId = abs(cellIds.x - cellId) < 0.1 ? cellIds.y : cellIds.x;
                return true;
            }
        }
    }
    return false;
}

vec3 getBarycentricCoords(float fid, vec3 p){
    vec3 indices = getIndices(fid);
    vec3 a = getPosition(indices.x);
    vec3 b = getPosition(indices.y);
    vec3 c = getPosition(indices.z);
    vec3 v0 = b - a, v1 = c - a, v2 = p - a;
    float d00 = dot(v0, v0);
    float d01 = dot(v0, v1);
    float d11 = dot(v1, v1);
    float d20 = dot(v2, v0);
    float d21 = dot(v2, v1);
    float denom = d00 * d11 - d01 * d01;
    float v = (d11 * d20 - d01 * d21) / denom;
    float w = (d00 * d21 - d01 * d20) / denom;
    float u = 1.0 - v - w;
    return vec3(w, u, v);
}

void main() {
    vec3 direction = origin - cameraPosition;
    float cellId = getCellIDs(originFaceId).x;
    float faceId = originFaceId;
    vec3 barycentricCoords = getBarycentricCoords(faceId, origin);
    float value = getValue(faceId, barycentricCoords);
    vec3 hitPos = origin;

    float sumValue = 0.0;
    for(int i = 0; i < 50; i++) {
        vec3 oldHitPos = hitPos;
        float oldValue = value;

        bool hit = next(origin, direction, cellId, faceId, hitPos, barycentricCoords);
        if(!hit){fragmentColor = vec4(1.0, 0.0, 1.0, 1.0); return;}

        value = getValue(faceId, barycentricCoords);
        sumValue += density * 0.5 * (value + oldValue) * distance(oldHitPos, hitPos);
        if(cellId > 999998.0 || sumValue >= 1.0)
        {
            fragmentColor = vec4(sumValue, 0.0, 1.0 - sumValue, 1.0);
            return;
        }
    }
    fragmentColor = vec4(1.0, 0.0, sumValue, 1.0);
}