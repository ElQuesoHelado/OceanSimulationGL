#version 460 core

in vec3 vDirection;

out vec4 FragColor;

uniform samplerCube uSkybox;

void main()
{
    FragColor = texture(uSkybox, normalize(vDirection));
}