#include "App.hpp"
#include "par_shapes.h"
#include "primitives/Primitive.hpp"
#include <GLFW/glfw3.h>
#include <algorithm>
#include <cstdlib>
#include <glm/geometric.hpp>
#include <imgui.h>
#include <print>
#include <utility>

glm::vec3 App::closest_hit_planes(float x, float y) {
  glm::vec3 nearPoint = glm::unProject(
      glm::vec3(x, height - y, 0.0f),
      camera.view(),
      proj,
      glm::vec4(0, 0, width, height));

  glm::vec3 farPoint = glm::unProject(
      glm::vec3(x, height - y, 1000.0f),
      camera.view(),
      proj,
      glm::vec4(0, 0, width, height));

  glm::vec3 origin = camera.position();
  glm::vec3 direction = glm::normalize(farPoint - nearPoint);

  float t_XZ = -origin.y / direction.y,
        t_YZ = -origin.x / direction.x,
        t_XY = -origin.z / direction.z;

  glm::vec3 hit_XZ = origin + t_XZ * direction,
            hit_YZ = origin + t_YZ * direction,
            hit_XY = origin + t_XY * direction;

  if (glm::distance(hit_XY, origin) <= glm::distance(hit_XZ, origin) &&
      glm::distance(hit_XY, origin) <= glm::distance(hit_YZ, origin))
    return hit_XY;
  if (glm::distance(hit_XZ, origin) <= glm::distance(hit_YZ, origin))
    return hit_XZ;
  return hit_YZ;
}

void App::process_input() {
  if (ImGui::GetIO().WantCaptureMouse)
    return;

  // Cambios en Mouse
  double x, y;
  glfwGetCursorPos(window, &x, &y);

  double mouse_dx = x - mouse_x;
  double mouse_dy = y - mouse_y;

  mouse_x = x;
  mouse_y = y;

  bool alt = glfwGetKey(window, GLFW_KEY_LEFT_ALT) == GLFW_PRESS;
  bool shift = glfwGetKey(window, GLFW_KEY_LEFT_SHIFT) == GLFW_PRESS;

  // Actualizar camara
  if (alt && glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS) {
    camera.orbit(mouse_dx, -mouse_dy);
    return;
  } else if (shift &&
             glfwGetMouseButton(window, GLFW_MOUSE_BUTTON_LEFT) == GLFW_PRESS) {
    camera.pan(mouse_dx, mouse_dy);
    return;
  }

  // Seleccionar
  if (!primitives.empty() &&
      ImGui::IsKeyPressed(ImGuiKey_2)) {
    if (selected) {
      std::swap(primitives[*selected].color, buffered_color);
      *selected = (*selected + 1) % primitives.size();
    } else {
      selected = 0;
    }

    std::swap(primitives[*selected].color, buffered_color);
  } else if (!primitives.empty() &&
             ImGui::IsKeyPressed(ImGuiKey_1)) {

    if (selected) {
      std::swap(primitives[*selected].color, buffered_color);
      *selected = (*selected + primitives.size() - 1) % primitives.size();
    } else {
      selected = primitives.size() - 1;
    }

    std::swap(primitives[*selected].color, buffered_color);
  } else if (ImGui::IsKeyPressed(ImGuiKey_GraveAccent)) {

    if (selected)
      std::swap(primitives[*selected].color, buffered_color);

    selected = std::nullopt;
  }

  if (!ImGui::IsMouseClicked(ImGuiMouseButton_Left))
    return;

  // Inserts
  ImVec2 pos = ImGui::GetMousePos();
  // auto &bf = in_pts_buf;

  // std::println("GLFW: {}, {}", x, y);
  // std::println("IMGU: {}, {}", pos.x, pos.y);

  // bf.emplace_back(pos.x, height - pos.y, 0);

  switch (curr_prim) {
  case Primitive::Type::Cube: {
    primitives.push_back(
        Primitive::Cube(closest_hit_planes(pos.x, pos.y),
                        curr_color, texture_manager["blank"]));

    break;
  }

  case Primitive::Type::Sphere:
    primitives.push_back(
        Primitive::Sphere(closest_hit_planes(pos.x, pos.y),
                          curr_color, texture_manager["brick"]));

    break;

  case Primitive::Type::Torus:
    primitives.push_back(
        Primitive::Torus(closest_hit_planes(pos.x, pos.y),
                         curr_color, texture_manager["brick"]));
    break;

  case Primitive::Type::Klein: {
    primitives.push_back(
        Primitive::Klein(closest_hit_planes(pos.x, pos.y),
                         curr_color, texture_manager["brick"]));
    break;
  }
  case Primitive::Type::Rock:
    primitives.push_back(
        Primitive::Rock(closest_hit_planes(pos.x, pos.y),
                        curr_color, texture_manager["brick"]));
    break;

  case Primitive::Type::Pen: {
    primitives.push_back(
        Primitive::Pen(closest_hit_planes(pos.x, pos.y),
                       curr_color, texture_manager["brick"]));
    break;
  }

  case Primitive::Type::Cylinder: {
    primitives.push_back(
        Primitive::Cylinder(closest_hit_planes(pos.x, pos.y),
                            curr_color, texture_manager["brick"]));
    break;
  }
  case Primitive::Type::Tetrahedron: {
    primitives.push_back(
        Primitive::Tetrahedron(closest_hit_planes(pos.x, pos.y),
                               curr_color, texture_manager["brick"]));
    break;
  }
  case Primitive::Type::Cone: {
    primitives.push_back(
        Primitive::Cone(closest_hit_planes(pos.x, pos.y),
                        curr_color, texture_manager["brick"]));
    break;
  }
  }
  // in_pts_buf.clear();
}
