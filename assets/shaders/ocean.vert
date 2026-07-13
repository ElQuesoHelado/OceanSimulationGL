#version 460 core
layout(location = 0) in vec3 aPos;
layout(location = 1) in vec3 aNormal;
layout(location = 2) in vec2 aTexCoord;

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
out vec3 Normal;

void main() {
    vec3 pos = aPos;

    float dHdx = 0.0;
    float dHdz = 0.0;

    float height = 0.0;
    for(int i = 0; i < waveCount; i++) {
        float k = (4.0 * 3.14159265 * 3.14159265 * waves[i].frequency * waves[i].frequency) / 9.81;
        float dirCos = cos(waves[i].direction);
        float dirSin = sin(waves[i].direction);
        float theta = k * (pos.x * dirCos + pos.z * dirSin) - 2.0 * 3.14159265 * waves[i].frequency * time + waves[i].phase;

        float c = cos(theta);
        float s = sin(theta);
        height += waves[i].amplitude * c;

        // Formula de vector normal usando derivadas respc (x,z)
        float dTheta = -waves[i].amplitude * s * k;
        dHdx += dTheta * dirCos;
        dHdz += dTheta * dirSin;
    }
    pos.y = height * 8;

    // Normal = aNormal;
    // Normal = vec3(0,1,0);
    Normal = normalize(vec3(-dHdx, 1.0, -dHdz)); // Formula

    FragPos = pos;

    // TexCoord = aPos.xz;
    // TexCoord = vec2(1,0);
    TexCoord = aTexCoord;
    gl_Position = uProjection * uView * vec4(pos, 1.0);
}
