#version 460 core

in vec3 worldPos;

uniform vec3 uColor;

out vec4 FragColor;

void main()
{
    float scale = 1.0;

    vec2 coord = worldPos.xz / scale;

    vec2 grid = abs(fract(coord - 0.5) - 0.5) / fwidth(coord);

    float line = min(grid.x, grid.y);

    float intensity = 1.0 - min(line, 1.0);

    FragColor = vec4(uColor * intensity, 1.0);
}
