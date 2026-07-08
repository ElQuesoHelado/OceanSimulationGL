#version 460 core

in vec2 TexCoord;

uniform sampler2D uTexture;
uniform vec4 uColor;

out vec4 FragColor;

void main()
{
    vec4 tex = texture(uTexture, TexCoord);
    
    FragColor = vec4(tex.rgb * uColor.rgb, tex.a);
}
