#version 460 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoord;

uniform mat4 uView;
uniform mat4 uProjection;
uniform mat4 uModel;

out vec2 TexCoord;

void main()
{
    // Extraer unicamente la traslacion del modelo
    vec3 center = vec3(uModel[3]);

    // Obtener los ejes de la camara
    vec3 right = vec3(
        uView[0][0],
        uView[1][0],
        uView[2][0]
    );

    vec3 up = vec3(
        uView[0][1],
        uView[1][1],
        uView[2][1]
    );

    // Obtener la escala del modelo
    float scaleX = length(vec3(uModel[0]));
    float scaleY = length(vec3(uModel[1]));

    vec3 worldPos =
        center +
        right * (aPos.x * scaleX) +
        up    * (aPos.y * scaleY);

    gl_Position = uProjection * uView * vec4(worldPos, 1.0);

    TexCoord = aTexCoord;
}
