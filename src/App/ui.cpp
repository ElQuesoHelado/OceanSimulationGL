#include "App.hpp"
#include "par_shapes.h"
#include "primitives/Primitive.hpp"
#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_opengl3.h>
#include <print>

void App::render_ui() {
  ImGui_ImplOpenGL3_NewFrame();
  ImGui_ImplGlfw_NewFrame();
  ImGui::NewFrame();

  const float toolbarWidth = 100.0f;

  ImGui::SetNextWindowPos(ImVec2(0, 0));
  ImGui::SetNextWindowSize(ImVec2(toolbarWidth, (float)height));

  ImGuiWindowFlags flags =
      ImGuiWindowFlags_NoMove |
      ImGuiWindowFlags_NoResize |
      ImGuiWindowFlags_NoCollapse;

  ImGui::Begin("Editor3D", nullptr, flags);

  ImGui::Text("Figuras");
  ImGui::Separator();

  if (ImGui::BeginTable("PrimitiveTable", 2)) {
    ImGui::TableNextRow();

    ImGui::TableSetColumnIndex(0);
    if (ImGui::Button("Cubo", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Cube;

    ImGui::TableSetColumnIndex(1);
    if (ImGui::Button("Sphe", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Sphere;

    ImGui::TableNextRow();

    ImGui::TableSetColumnIndex(0);
    if (ImGui::Button("Toru", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Torus;

    ImGui::TableSetColumnIndex(1);
    if (ImGui::Button("Botl", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Klein;

    ImGui::TableNextRow();

    ImGui::TableSetColumnIndex(0);
    if (ImGui::Button("Roca", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Rock;

    ImGui::TableSetColumnIndex(1);
    if (ImGui::Button("Pen", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Pen;

    ImGui::TableNextRow();

    ImGui::TableSetColumnIndex(0);
    if (ImGui::Button("Cilind", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Cylinder;

    ImGui::TableSetColumnIndex(1);
    if (ImGui::Button("Tetra", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Tetrahedron;

    ImGui::TableNextRow();

    ImGui::TableSetColumnIndex(0);
    if (ImGui::Button("Cone", ImVec2(-FLT_MIN, 0)))
      curr_prim = Primitive::Type::Cone;

    ImGui::EndTable();
  }

  ImGui::Separator();

  ImGui::ColorEdit3("Col", glm::value_ptr(curr_color));

  if (ImGui::Button("Paint")) {
    if (selected) {
      std::swap(primitives[*selected].color, buffered_color);
      primitives[*selected].color = curr_color;
      selected = std::nullopt;
    }
  }

  ImGui::Separator();

  // ImGui::Separator();
  ImGui::Text("Operaciones");

  ImGui::Checkbox("Centered", &centered_ops);

  //==========
  // Scales
  //==========

  ImGui::Text("SCL");
  ImGui::SameLine();
  if (ImGui::Button("+##l1_plus")) {
    if (selected)
      primitives[*selected].scale({1.1, 1.1, 1.1});
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l1_minus")) {
    if (selected)
      primitives[*selected].scale({.9, .9, .9});
  }

  //==========
  // Rotaciones
  //==========

  ImGui::Text("ROTX");
  ImGui::SameLine();
  if (ImGui::Button("+##l2_plus")) {
    if (selected)
      primitives[*selected].rotate(.3, {1, 0, 0}, centered_ops);
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l2_minus")) {
    if (selected)
      primitives[*selected].rotate(-.3, {1, 0, 0}, centered_ops);
  }
  ImGui::Text("ROTY");
  ImGui::SameLine();
  if (ImGui::Button("+##l3_plus")) {
    if (selected)
      primitives[*selected].rotate(.3, {0, 1, 0}, centered_ops);
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l3_minus")) {
    if (selected)
      primitives[*selected].rotate(-.3, {0, 1, 0}, centered_ops);
  }
  ImGui::Text("ROTZ");
  ImGui::SameLine();
  if (ImGui::Button("+##l4_plus")) {
    if (selected)
      primitives[*selected].rotate(.3, {0, 0, 1}, centered_ops);
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l4_minus")) {
    if (selected)
      primitives[*selected].rotate(-.3, {0, 0, 1}, centered_ops);
  }

  //==========
  // Translates
  //==========

  ImGui::Text("TRANSX");
  ImGui::SameLine();
  if (ImGui::Button("+##l5_plus")) {
    if (selected)
      primitives[*selected].translate({1, 0, 0});
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l5_minus")) {
    if (selected)
      primitives[*selected].translate({-1, 0, 0});
  }
  ImGui::Text("TRANSY");
  ImGui::SameLine();
  if (ImGui::Button("+##l6_plus")) {
    if (selected)
      primitives[*selected].translate({0, 1, 0});
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l6_minus")) {
    if (selected)
      primitives[*selected].translate({0, -1, 0});
  }
  ImGui::Text("TRANSZ");
  ImGui::SameLine();
  if (ImGui::Button("+##l7_plus")) {
    if (selected)
      primitives[*selected].translate({0, 0, 1});
  }
  ImGui::SameLine();
  if (ImGui::Button("-##l7_minus")) {
    if (selected)
      primitives[*selected].translate({0, 0, -1});
  }

  //==========
  // Miscs
  //==========

  ImGui::Separator();
  ImGui::Text("MISCS");
  if (ImGui::Checkbox("Wireframe", &wireframe)) {
    glPolygonMode(
        GL_FRONT_AND_BACK,
        wireframe ? GL_LINE : GL_FILL);
  }
  if (ImGui::Checkbox("Lighting", &enable_lighting)) {
  }
  if (ImGui::Button("DUPE")) {
    if (selected) {
      std::swap(primitives[*selected].color, buffered_color);
      primitives.push_back(primitives[*selected]);
      selected = std::nullopt;
    }
  }
  if (ImGui::Button("DEL")) {
    if (selected) {
      std::swap(primitives[*selected].color, buffered_color);
      primitives.erase(primitives.begin() + *selected);
      selected = std::nullopt;
    }
  }

  ImGui::Separator();
  ImGui::Text("Textures");

  if (ImGui::BeginListBox("##Textures", ImVec2(-FLT_MIN, 8 * ImGui::GetTextLineHeightWithSpacing()))) {
    for (const auto &texture_name : texture_names) {
      if (ImGui::Selectable(texture_name.c_str(), false)) {
        if (selected)
          primitives[*selected].texture = texture_manager[texture_name];
      }
    }

    ImGui::EndListBox();
  }

  ImGui::End();

  ImGui::Render();
  ImGui_ImplOpenGL3_RenderDrawData(ImGui::GetDrawData());
}
