#include "ocean/ocean.hpp"

void Ocean::createGLObjects() {
  glCreateVertexArrays(1, &VAO);
  glCreateBuffers(1, &VBO);
  glCreateBuffers(1, &texCoordsVBO);
  glCreateBuffers(1, &EBO);
}

void Ocean::setupVAO() {
  // posiciones
  glVertexArrayVertexBuffer(VAO, 0, VBO, 0, sizeof(glm::vec3));
  glEnableVertexArrayAttrib(VAO, 0);
  glVertexArrayAttribFormat(VAO, 0, 3, GL_FLOAT, GL_FALSE, 0);
  glVertexArrayAttribBinding(VAO, 0, 0);

  // coordenadas UV
  glVertexArrayVertexBuffer(VAO, 1, texCoordsVBO, 0, sizeof(glm::vec2));
  glEnableVertexArrayAttrib(VAO, 1);
  glVertexArrayAttribFormat(VAO, 1, 2, GL_FLOAT, GL_FALSE, 0);
  glVertexArrayAttribBinding(VAO, 1, 1);
}

void Ocean::initialize() {
  shader->use();
  createGLObjects();
  setupVAO();
  load_waves_p_uniforms();
}

void Ocean::load_waves_p_uniforms() {
  for (size_t i = 0; i < waves.size(); i++) {
    shader->setFloat("waves[" + std::to_string(i) + "].amplitude",
                     waves[i].amplitude);

    shader->setFloat("waves[" + std::to_string(i) + "].frequency",
                     waves[i].frequency);

    shader->setFloat("waves[" + std::to_string(i) + "].direction",
                     waves[i].direction);

    shader->setFloat("waves[" + std::to_string(i) + "].phase",
                     waves[i].phase);
  }

  shader->setInt("waveCount", waves.size());
}

void Ocean::destroy() {
  if (VAO)
    glDeleteVertexArrays(1, &VAO);
  if (VBO)
    glDeleteBuffers(1, &VBO);
  if (texCoordsVBO)
    glDeleteBuffers(1, &texCoordsVBO);
  if (EBO)
    glDeleteBuffers(1, &EBO);

  VAO = VBO = EBO = 0;
}
