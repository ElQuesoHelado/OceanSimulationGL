#include "ocean/Ocean.hpp"

void Ocean::createGLObjects() {
  glCreateVertexArrays(1, &VAO);
  glCreateBuffers(1, &VBO);
  glCreateBuffers(1, &normalsVBO);
  glCreateBuffers(1, &texCoordsVBO);
  glCreateBuffers(1, &EBO);
}