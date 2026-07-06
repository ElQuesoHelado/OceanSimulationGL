#include "ocean/ocean.hpp"

void Ocean::draw() {
  shader->use();
  shader->setFloat("time", time);

  glBindVertexArray(VAO);

  glBindTexture(GL_TEXTURE_2D, texture->id);

  glDrawElements(
      GL_TRIANGLES,
      indices.size(),
      GL_UNSIGNED_INT,
      nullptr);
  time += .1;
}
