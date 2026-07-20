#version 460 core

layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoord;

uniform mat4 uView;
uniform mat4 uProjection;

out vec3 TexCoords;

void main()
{
    TexCoords = aPos;

    mat4 view = mat4(mat3(uView)); // Elimina Traslacion

    vec4 pos = uProjection * view * vec4(aPos, 1.0);

    gl_Position = pos.xyww;
}
