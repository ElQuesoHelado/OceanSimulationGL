#pragma once

#include <glm/ext/vector_float2.hpp>
#include <glm/ext/vector_float3.hpp>
#include <vector>

#include <glad/glad.h>

class Mesh {
private:
  GLuint VAO;
  GLuint VBO;

  GLsizei point_count{};

public:
  Mesh();

  // Sin copia
  Mesh(const Mesh &) = delete;
  Mesh &operator=(const Mesh &) = delete;

  Mesh(Mesh &&other) noexcept;
  Mesh &operator=(Mesh &&other) noexcept;

  // void upload(const std::vector<glm::vec2> &a, const std::vector<glm::vec2> &b);
  void upload(const std::vector<glm::vec3> &a, const std::vector<glm::vec3> &b);

  ~Mesh();

  void draw() const;
};
