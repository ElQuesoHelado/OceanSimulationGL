#include "App.hpp"
#include "par_shapes.h"
#include "primitives/Primitive.hpp"
#include <glm/detail/qualifier.hpp>
#include <glm/ext/scalar_constants.hpp>
#include <glm/ext/vector_float3.hpp>
#include <glm/geometric.hpp>
#include <glm/gtc/constants.hpp>
#include <glm/trigonometric.hpp>
#include <memory>
#include <print>

void App::run() {
  while (!glfwWindowShouldClose(window)) {
    if (glfwGetKey(window, GLFW_KEY_ESCAPE) == GLFW_PRESS)
      glfwSetWindowShouldClose(window, true);

    glClearColor(0.1f, 0.1f, 0.1f, 1.0f);
    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);

    shader->use();
    shader->setMat4("uView", camera.view());

    // glm::vec3 lightPos(3.0f, 5.0f, 2.0f);
    glm::vec3 lightPos({20, 20, 30});

    // glm::vec3 lightPos(15, 20, 15);

    shader->setBool("uLightingEnabled", enable_lighting);
    shader->setVec3("lightPos", lightPos);
    shader->setVec3("lightColor", glm::vec3(1));
    shader->setVec3("viewPos", camera.position());

    for (auto &p : primitives) {
      shader->setVec3("uColor", p.color);
      shader->setFloat("shininess", p.shininess);
      p.draw();
    }

    shader->setVec3("uColor", scene->color);
    scene->draw_gizmo();

    //
    // Render de piso
    //
    if (wireframe)
      glPolygonMode(GL_FRONT_AND_BACK, GL_FILL);

    floor_shader->use();

    floor_shader->setMat4("uProjection", proj);
    floor_shader->setMat4("uView", camera.view());
    floor_shader->setVec3("uColor", {.3f, 1.f, .3f});

    scene->draw_floor();

    if (wireframe)
      glPolygonMode(GL_FRONT_AND_BACK, GL_LINE);

    render_ui();
    process_input();

    glfwSwapBuffers(window);
    glfwPollEvents();
  }
}
