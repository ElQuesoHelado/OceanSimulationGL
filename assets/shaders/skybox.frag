#version 460 core
in vec3 TexCoords;

uniform samplerCube uSkybox;

out vec4 FragColor;

void main()
{
    FragColor = texture(uSkybox, TexCoords);
}
