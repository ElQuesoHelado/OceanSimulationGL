#version 460 core

layout(location = 0) in vec3 aPosition;

out vec3 vDirection;

uniform mat4 uProjection;
uniform mat4 uView;

void main()
{
    vDirection = aPosition;

    mat4 view = mat4(mat3(uView));

    vec4 pos = uProjection * view * vec4(aPosition, 1.0);

    gl_Position = pos.xyww;
}