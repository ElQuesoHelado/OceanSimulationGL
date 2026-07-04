#pragma once

#include <glm/ext/vector_float2.hpp>
#include <glm/ext/vector_float3.hpp>
#include <vector>

struct Shape {
  // Real points son la figura como tal
  // Pixel points son generadas por Bresenham
  std::vector<glm::vec3> real_points, pixel_points, pixel_fill{};

  // Boundary box
  glm::vec2 interv_x, interv_y;

  glm::vec3 color{1, 1, 1};
};
