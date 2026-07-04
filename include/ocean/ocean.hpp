#pragma once

#include "Shader.hpp"
#include <cstddef>
#include <glad/glad.h>
#include <glm/ext/vector_float2.hpp>
#include <glm/ext/vector_float3.hpp>
#include <glm/ext/vector_float4.hpp>

#include <memory>
#include <vector>

struct Wave {
  float amplitude, frequency, direction, phase;
};