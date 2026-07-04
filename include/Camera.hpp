#pragma once

#include <glm/ext/matrix_float4x4.hpp>
#include <glm/ext/matrix_transform.hpp>
#include <glm/ext/vector_float3.hpp>
#include <glm/trigonometric.hpp>

struct Camera {
  glm::vec3 target{0, 0, 0};

  // No se guarda directamente posicion de camara
  float distance = 50.f;

  float yaw = 45.f, pitch = 20.f;

  glm::mat4 view() const {
    float ry = glm::radians(yaw);
    float rp = glm::radians(pitch);

    glm::vec3 pos;

    pos.x = target.x + distance * cos(rp) * cos(ry);
    pos.y = target.y + distance * sin(rp);
    pos.z = target.z + distance * cos(rp) * sin(ry);

    return glm::lookAt(
        pos,
        target,
        glm::vec3(0, 1, 0));
  }

  glm::vec3 position() const {
    float ry = glm::radians(yaw);
    float rp = glm::radians(pitch);

    return {
        target.x + distance * cos(rp) * cos(ry),
        target.y + distance * sin(rp),
        target.z + distance * cos(rp) * sin(ry)};
  }

  void orbit(float dx, float dy) {
    yaw += dx * 0.3f;
    pitch -= dy * 0.3f;

    pitch = glm::clamp(pitch, -89.f, 89.f);
  }

  void zoom(float delta) {
    distance *= (1.f - delta * 0.1f);
    distance = std::max(distance, 0.1f);
  }

  void pan(float dx, float dy) {
    glm::vec3 forward = glm::normalize(target - position());
    glm::vec3 right = glm::normalize(glm::cross(forward, glm::vec3(0, 1, 0)));
    glm::vec3 up = glm::normalize(glm::cross(right, forward));

    float speed = distance * 0.001f;

    target -= right * dx * speed;
    target += up * dy * speed;
  }
};
