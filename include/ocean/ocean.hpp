#pragma once

#include "Shader.hpp"
#include <cstddef>
#include <glad/glad.h>
#include <glm/ext/vector_float2.hpp>
#include <glm/ext/vector_float3.hpp>
#include <glm/ext/vector_float4.hpp>

#include <memory>
#include <vector>

struct Wave {
  float amplitude, frequency, direction, phase;
};

struct Ocean {
  float time{};
  size_t npoints;
  std::vector<glm::vec3> points;
  std::vector<Wave> waves;
  std::vector<GLuint> indices;

  std::unique_ptr<Shader> shader;

  GLuint VAO{}, VBO{}, normalsVBO{}, texCoordsVBO{}, EBO{};

  Ocean(size_t n_points)
      : npoints(n_points) {
    points.reserve(n_points);

    for (size_t i{}; i < npoints; ++i) {
      for (size_t j{}; j < npoints; ++j) {
        points.emplace_back(i, 0.f, j);
      }
    }

    indices.reserve((npoints - 1) * (npoints - 1) * 6);

    for (size_t i = 0; i < npoints - 1; i++) {
      for (size_t j = 0; j < npoints - 1; j++) {

        GLuint v0 = i * npoints + j;
        GLuint v1 = v0 + 1;
        GLuint v2 = (i + 1) * npoints + j;
        GLuint v3 = v2 + 1;

        indices.push_back(v0);
        indices.push_back(v2);
        indices.push_back(v1);

        indices.push_back(v1);
        indices.push_back(v2);
        indices.push_back(v3);
      }
    }

    waves.push_back({0.35f, 0.20f, 0.00f, 0.00f});
    waves.push_back({0.20f, 0.35f, 0.785f, 1.57f});
    waves.push_back({0.15f, 0.50f, 2.094f, 3.14f});
    waves.push_back({0.10f, 0.15f, 4.189f, 0.78f});

    shader = std::make_unique<Shader>("shaders/ocean.vert", "shaders/ocean.frag");

    initialize();
    upload();
  }

  ~Ocean() {
    destroy();
  }

  void createGLObjects();
  void setupVAO();
  void initialize();
  void destroy();

  void load_waves_p_uniforms();

  void upload() {
    glNamedBufferData(
        VBO,
        points.size() * sizeof(glm::vec3),
        points.data(),
        GL_STATIC_DRAW);

    glNamedBufferData(
        EBO,
        indices.size() * sizeof(GLuint),
        indices.data(),
        GL_STATIC_DRAW);

    glVertexArrayElementBuffer(VAO, EBO);
  }

  void draw();
};