#include "App.hpp"

void App::resize() {
  int fbw, fbh;
  glfwGetFramebufferSize(window, &fbw, &fbh);

  int ww, wh;
  glfwGetWindowSize(window, &ww, &wh);
  width = ww;
  height = wh;

  glViewport(0, 0, fbw, fbh);

  // 2D
  // glm::mat4 proj = glm::ortho(
  //     0.0f,
  //     (float)fbw,
  //     0.0f,
  //     (float)fbh,
  //     -1000.0f,
  //     1000.0f);

  // glm::mat4 view = 1;

  proj =
      glm::perspective(
          glm::radians(80.0f),
          (float)fbw / fbh,
          .1f,
          1000.f);

  shader->use();
  shader->setMat4("uProjection", proj);
}
