#version 460 core

layout(location = 0) in vec3 aPos;
// layout(location = 1) in vec3 aNormal;
// layout(location = 2) in vec2 aTexCoord;

struct Wave
{
    float amplitude;
    float frequency;
    float direction;
    float phase;
};

const int MAX_WAVES = 16;

uniform Wave waves[MAX_WAVES];
uniform int waveCount;

uniform float time;

uniform mat4 uProjection;
uniform mat4 uView;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoord;

void main()
{
}
