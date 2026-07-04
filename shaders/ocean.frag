#version 460 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

uniform sampler2D uTexture;
uniform vec3 uColor;

uniform vec3 lightPos;
uniform vec3 lightColor;

// Brillos
uniform vec3 viewPos;

uniform float shininess;

uniform bool uLightingEnabled;

out vec4 FragColor;

void main()
{
  vec4 texColor = texture(uTexture, TexCoord);
  vec3 baseColor = mix(uColor, texColor.rgb, 0.7);

  vec3 result;


}