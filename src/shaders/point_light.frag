
layout (location = 0) out vec4 color;

//uniform samplerCube shadowCubeMap;
/*uniform mat4 shadowMVP0;
uniform mat4 shadowMVP1;
uniform mat4 shadowMVP2;
uniform mat4 shadowMVP3;
uniform mat4 shadowMVP4;
uniform mat4 shadowMVP5;*/

/*vec3 calculate_point_light(vec3 position)
{
    vec3 color = calculate_attenuated_light(pointLight.base, pointLight.attenuation, pointLight.position, position);

    mat4 shadowMatrix;
    float x = abs(lightDirection.x);
    float y = abs(lightDirection.y);
    float z = abs(lightDirection.z);
    if(x > y && x > z)
    {
        if(lightDirection.x > 0.0)
        {
            shadowMatrix = shadowMVP0;
        }
        else {
            shadowMatrix = shadowMVP1;
        }
    }
    else if(y > x && y > z)
    {
        if(lightDirection.y > 0.0)
        {
            shadowMatrix = shadowMVP2;
        }
        else {
            shadowMatrix = shadowMVP3;
        }
    }
    else if(z > x && z > y)
    {
        if(lightDirection.z > 0.0)
        {
            shadowMatrix = shadowMVP4;
        }
        else {
            shadowMatrix = shadowMVP5;
        }
    }
    else {
        return vec4(1., 0., 0., 1.);
    }

    float shadow = 1.f;
    vec4 shadow_coord = shadowMatrix * vec4(position, 1.);
    if ( texture(shadowCubeMap, lightDirection).x < (shadow_coord.z - 0.005)/shadow_coord.w)
    {
        shadow = 0.5f;
    }

    return color;
}*/

void main()
{
    color = vec4(calculate_point_light(get_surface()), 1.0);
}
