#pragma once

#include "Shader.hpp"
#include "Texture.hpp"
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
  std::vector<glm::vec2> texCoords;

  std::unique_ptr<Shader> shader;
  std::shared_ptr<Texture> texture;

  GLuint VAO{}, VBO{}, texCoordsVBO{}, EBO{};

  Ocean(size_t n_points, const std::shared_ptr<Texture> &tex)
      : npoints(n_points), texture(tex) {
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

    texCoords.reserve(n_points * n_points);
    for (size_t i{}; i < npoints; ++i)
      for (size_t j{}; j < npoints; ++j)
        texCoords.emplace_back(
            float(i) / float(npoints - 1),
            float(j) / float(npoints - 1));

    // waves.push_back({0.35f, 0.20f, 0.00f, 0.00f});
    // waves.push_back({0.20f, 0.35f, 0.785f, 1.57f});
    // waves.push_back({0.15f, 0.50f, 2.094f, 3.14f});
    // waves.push_back({0.10f, 0.15f, 4.189f, 0.78f});

    waves.push_back({0.6f, 0.08f, 0.0f, 0.0f});
    waves.push_back({0.35f, 0.12f, 0.4f, 1.3f});
    waves.push_back({0.18f, 0.18f, -0.3f, 2.7f});
    waves.push_back({0.09f, 0.28f, 0.7f, 0.5f});

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
        texCoordsVBO,
        texCoords.size() * sizeof(glm::vec2),
        texCoords.data(),
        GL_STATIC_DRAW);

    glNamedBufferData(
        EBO,
        indices.size() * sizeof(GLuint),
        indices.data(),
        GL_STATIC_DRAW);

    glVertexArrayElementBuffer(VAO, EBO);
  }

  void draw(float deltaTime);
};
