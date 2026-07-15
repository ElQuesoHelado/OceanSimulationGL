#version 460 core

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoord;

uniform sampler2D uTexture;
uniform vec4 uColor;

uniform vec3 uLightPos;
uniform vec3 uLightColor;

// Brillos
uniform vec3 uEye;

uniform float uShininess;

uniform bool uLightingEnabled;

out vec4 FragColor;

void main()
{
  vec4 texColor = texture(uTexture, TexCoord);
  vec3 baseColor = mix(uColor.rgb, texColor.rgb, 0.7);

  vec3 result;

  if(uLightingEnabled){
    // Ambiental
    float ambientStrength = 0.5;
    vec3 ambient = ambientStrength * uLightColor;

    // Difusa
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(uLightPos - FragPos);

    float diff = max(dot(norm, lightDir), 0.0);
    float diffuseStrength = 0.7;
    vec3 diffuse = diffuseStrength * diff * uLightColor;

    // Especular (Blinn-Phong)
    vec3 viewDir = normalize(uEye - FragPos);

    vec3 halfwayDir = normalize(lightDir + viewDir);

    float spec = pow(
        max(dot(norm, halfwayDir), 0.0),
        uShininess
    );

    vec3 specular =
        1 * spec * uLightColor;

    result =
        (ambient + diffuse) * baseColor +
        specular;
  }else{
    result = baseColor;
  }


  FragColor = vec4(result, texColor.a * uColor.a);
  // FragColor = vec4(TexCoord, 0.0, 1.0);
  // FragColor = vec4(1,1,1,1);
}
