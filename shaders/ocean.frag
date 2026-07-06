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

  if(uLightingEnabled){
    // Ambiental
    float ambientStrength = 0.4;
    vec3 ambient = ambientStrength * lightColor;

    // Difusa
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(lightPos - FragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    float diffuseStrength = 2;
    vec3 diffuse = diffuseStrength * diff * lightColor;

    // Especular (Blinn-Phong)
    vec3 viewDir = normalize(viewPos - FragPos);

    vec3 halfwayDir = normalize(lightDir + viewDir);

    float spec = pow(
        max(dot(norm, halfwayDir), 0.0),
        shininess
    );

    vec3 specular =
        2 * spec * lightColor;

    result =
        (ambient + diffuse) * baseColor +
        specular;
  }else{
    result = baseColor;
  }


  FragColor = vec4(result, texColor.a);
}
