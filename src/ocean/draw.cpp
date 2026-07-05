#include "ocean/ocean.hpp"

void Ocean::draw() {
  shader->use();
  shader->setFloat("time", time);

  glBindVertexArray(VAO);
  glDrawElements(
      GL_TRIANGLES,
      indices.size(),
      GL_UNSIGNED_INT,
      nullptr);
  time += .1;
}
