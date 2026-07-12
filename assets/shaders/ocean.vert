#version 460 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;

uniform mat4 uProjection;
uniform mat4 uView;

out vec3 FragPos;
out vec2 TexCoord;

void main() {
    vec3 pos = aPos;
    FragPos = pos;
    TexCoord = aTexCoord;
    gl_Position = uProjection * uView * vec4(pos, 1.0);
}