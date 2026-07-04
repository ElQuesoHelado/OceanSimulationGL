#pragma once

#include <cstddef>
#include <glm/ext/matrix_float4x4.hpp>
#include <glm/ext/vector_float2.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <vector>

struct MOps {
  glm::mat4 transform{1.0f};

  MOps &clear() {
    transform = glm::mat4(1.0f);
    return *this;
  }

  MOps &rotate(float deg) {
    transform = glm::rotate(
        transform,
        glm::radians(deg),
        glm::vec3(0.0f, 0.0f, 1.0f));
    return *this;
  }

  MOps &scale(glm::vec3 s) {
    transform = glm::scale(
        transform,
        glm::vec3(s.x, s.y, s.z));
    return *this;
  }

  MOps &trans(glm::vec3 t) {
    transform = glm::translate(
        transform,
        glm::vec3(t.x, t.y, t.z));
    return *this;
  }

  MOps &reflex_x() {
    transform = glm::scale(
        transform,
        glm::vec3(1, -1, 1));
    return *this;
  }

  MOps &reflex_y() {
    transform = glm::scale(
        transform,
        glm::vec3(-1, 1, 1));
    return *this;
  }

  MOps &reflex_xy() {
    transform *= glm::mat4(
        0, 1, 0, 0,
        1, 0, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1);

    return *this;
  }

  // Respecto a cualquier recta por el origen
  MOps &reflex(float deg) {
    return rotate(-deg).reflex_x().rotate(deg);
  }

  std::vector<glm::vec3> apply_new(const std::vector<glm::vec3> &points) {
    std::vector<glm::vec3> res;
    res.reserve(points.size());

    for (auto &p : points)
      res.emplace_back(transform * glm::vec4(p, 1.0f));

    return res;
  }

  void apply(std::vector<glm::vec3> &points) {
    for (auto &p : points)
      p = transform * glm::vec4(p, 1.0f);
  }

  void apply(std::vector<float> &points) {
    for (size_t i{}; i < points.size(); i += 3) {
      glm::vec3 p(points[i], points[i + 1], points[i + 2]);
      p = transform * glm::vec4(p, 1.0f);

      points[i] = p.x;
      points[i + 1] = p.y;
      points[i + 2] = p.z;
    }
  }

  // TODO: Solo concatena puntos pixel
  // static void concat() {}
};
