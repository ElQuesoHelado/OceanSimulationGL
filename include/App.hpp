#pragma once

#include <glad/glad.h>

#include <GLFW/glfw3.h>
#include <glm/ext/vector_float3.hpp>
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>
#include <memory>
#include <optional>
#include <unordered_map>
#include <vector>

#include "Camera.hpp"
#include "Shader.hpp"

#include "Texture.hpp"
#include "primitives/Primitive.hpp"

#include "Scene.hpp"

struct App {
  GLFWwindow *window;
  std::unique_ptr<Shader> shader, floor_shader;

  glm::mat4 proj;
  Camera camera;

  std::unique_ptr<Scene> scene;

  int width = 800, height = 600;

  std::vector<Primitive> primitives{};

  void render_ui();
  void process_input();

  glm::vec3 closest_hit_planes(float x, float y);

  double mouse_x{}, mouse_y{};
  glm::vec3 curr_color{1, 1, 1},
      buffered_color{1, 0, 0};
  Primitive::Type curr_prim{};

  std::optional<size_t> selected;

  bool wireframe{}, centered_ops{};

  // Luces
  bool enable_lighting{};

  // Texturas
  std::vector<std::string> texture_names;
  std::unordered_map<std::string, std::shared_ptr<Texture>> texture_manager;
  void load_textures();

  void resize();
  void run();

  App();
  ~App();
};
