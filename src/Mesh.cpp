#include "data/Mesh.hpp"
#include <glm/ext/vector_float3.hpp>

Mesh::Mesh() {
  glCreateVertexArrays(1, &VAO);
  glCreateBuffers(1, &VBO);

  glVertexArrayVertexBuffer(
      VAO,
      0,
      VBO,
      0,
      sizeof(glm::vec3));

  glEnableVertexArrayAttrib(VAO, 0);

  glVertexArrayAttribFormat(
      VAO,
      0,
      3,
      GL_FLOAT,
      GL_FALSE,
      0);

  glVertexArrayAttribBinding(
      VAO,
      0,
      0);
}

Mesh::Mesh(Mesh &&other) noexcept {
  VAO = other.VAO;
  VBO = other.VBO;
  point_count = other.point_count;

  other.VAO = 0;
  other.VBO = 0;
  other.point_count = 0;
}

Mesh &Mesh::operator=(Mesh &&other) noexcept {
  if (this != &other) {
    // Limpia recursos actuales
    glDeleteVertexArrays(1, &VAO);
    glDeleteBuffers(1, &VBO);

    VAO = other.VAO;
    VBO = other.VBO;
    point_count = other.point_count;

    other.VAO = 0;
    other.VBO = 0;
    other.point_count = 0;
  }

  return *this;
}

void Mesh::upload(const std::vector<glm::vec3> &a, const std::vector<glm::vec3> &b) {
  point_count = a.size() + b.size();

  std::vector<glm::vec3> temp;
  temp.reserve(point_count);

  temp.insert(temp.end(), a.begin(), a.end());
  temp.insert(temp.end(), b.begin(), b.end());

  glNamedBufferData(
      VBO,
      temp.size() * sizeof(glm::vec3),
      temp.data(),
      GL_STATIC_DRAW);
}

Mesh::~Mesh() {
  if (VAO)
    glDeleteVertexArrays(1, &VAO);

  if (VBO)
    glDeleteBuffers(1, &VBO);
}

void Mesh::draw() const {
  glBindVertexArray(VAO);

  glDrawArrays(GL_POINTS, 0, point_count);
}
