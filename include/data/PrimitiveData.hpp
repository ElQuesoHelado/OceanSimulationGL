#pragma once

#include "Mesh.hpp"
#include "Shape.hpp"
#include <cmath>

struct PrimitiveData {
  Shape shape;
  Mesh mesh;

  static void border(std::vector<glm::vec3> &pixel_points, float x, float y) {
    for (int i{0}; i < 50; ++i) {
      pixel_points.emplace_back(std::round(x), std::round(y), i);
    }
  }
};
