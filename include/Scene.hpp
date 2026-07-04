#pragma once

#include <glm/ext/vector_float3.hpp>
#include <vector>

#include <glad/glad.h>

struct Scene {
  GLuint VAO{}, VBO{};
  GLuint VAO_floor{}, VBO_floor{};
  std::vector<glm::vec3> giz_pts;
  glm::vec3 color{0, 1, 1};

  std::vector<glm::vec3> floor_vertices = {
      {0.0f, 0.0f, 0.0f},
      {1000.0f, 0.0f, 0.0f},
      {1000.0f, 0.0f, 1000.0f},

      {0.0f, 0.0f, 0.0f},
      {1000.0f, 0.0f, 1000.0f},
      {0.0f, 0.0f, 1000.0f}};

  Scene() {
    giz_pts = {
        {0, 0, 0},
        {1000, 0, 0},
        {0, 0, 0},
        {0, 1000, 0},
        {0, 0, 0},
        {0, 0, 1000},
    };

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

    glNamedBufferData(
        VBO,
        giz_pts.size() * sizeof(glm::vec3),
        giz_pts.data(),
        GL_STATIC_DRAW);

    // Floor
    glCreateVertexArrays(1, &VAO_floor);
    glCreateBuffers(1, &VBO_floor);

    glVertexArrayVertexBuffer(
        VAO_floor,
        0,
        VBO_floor,
        0,
        sizeof(glm::vec3));

    glEnableVertexArrayAttrib(VAO_floor, 0);

    glVertexArrayAttribFormat(
        VAO_floor,
        0,
        3,
        GL_FLOAT,
        GL_FALSE,
        0);

    glVertexArrayAttribBinding(
        VAO_floor,
        0,
        0);

    glNamedBufferData(
        VBO_floor,
        floor_vertices.size() * sizeof(glm::vec3),
        floor_vertices.data(),
        GL_STATIC_DRAW);
  };

  void draw_gizmo() const {
    glBindVertexArray(VAO);
    glDrawArrays(GL_LINES, 0, giz_pts.size());
  }

  void draw_floor() const {
    glBindVertexArray(VAO_floor);
    glDrawArrays(GL_TRIANGLES, 0, floor_vertices.size());
  }
};
