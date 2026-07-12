#version 460 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec2 aTexCoord;

struct Wave {
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
out vec2 TexCoord;

void main() {
    vec3 pos = aPos;

    float height = 0.0;
    for(int i = 0; i < waveCount; i++) {
        float k = (4.0 * 3.14159265 * 3.14159265 * waves[i].frequency * waves[i].frequency) / 9.81;
        float dirCos = cos(waves[i].direction);
        float dirSin = sin(waves[i].direction);
        float theta = k * (pos.x * dirCos + pos.z * dirSin) - 2.0 * 3.14159265 * waves[i].frequency * time + waves[i].phase;

        float c = cos(theta);
        float s = sin(theta);
        height += waves[i].amplitude * c;
    }
    pos.y = height;

    FragPos = pos;
    TexCoord = aTexCoord;
    gl_Position = uProjection * uView * vec4(pos, 1.0);
}