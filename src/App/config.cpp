#include "App.hpp"
#include <filesystem>
#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_opengl3.h>
#include <iostream>
#include <memory>

void framebuffer_size_callback(GLFWwindow *window, int width, int height) {
  auto *app = static_cast<App *>(glfwGetWindowUserPointer(window));

  app->resize();
}

void scroll_callback(GLFWwindow *window,
                     double xoffset,
                     double yoffset) {
  auto *app = static_cast<App *>(glfwGetWindowUserPointer(window));

  if (ImGui::GetIO().WantCaptureMouse)
    return;

  app->camera.zoom(yoffset);
}

App::App() {
  glfwInit();
  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
  glfwWindowHint(GLFW_OPENGL_PROFILE, GLFW_OPENGL_CORE_PROFILE);

  window = glfwCreateWindow(800, 600, "Puntos", nullptr, nullptr);
  if (!window) {
    std::cerr << "No se pudo crear la ventana\n";
    glfwTerminate();
    return;
  }
  glfwMakeContextCurrent(window);
  glfwSetWindowUserPointer(window, this);
  glfwSetFramebufferSizeCallback(window, framebuffer_size_callback);
  glfwSetScrollCallback(window, scroll_callback);

  if (!gladLoadGLLoader((GLADloadproc)glfwGetProcAddress)) {
    std::cerr << "Error cargando GLAD\n";
    return;
  }

  shader = std::make_unique<Shader>("shaders/shader.vert", "shaders/shader.frag");
  floor_shader = std::make_unique<Shader>("shaders/floor.vert", "shaders/floor.frag");

  resize();

  // Init ImGui
  IMGUI_CHECKVERSION();
  ImGui::CreateContext();

  ImGuiIO &io = ImGui::GetIO();

  ImGui::StyleColorsDark();

  ImGui_ImplGlfw_InitForOpenGL(window, true);
  ImGui_ImplOpenGL3_Init("#version 460");

  // TODO: Afecta el draw de grid
  // glEnable(GL_CULL_FACE);
  // glCullFace(GL_BACK);
  glEnable(GL_DEPTH_TEST);

  scene = std::make_unique<Scene>();

  // Carga de texturas
  load_textures();
}

void App::load_textures() {
  namespace fs = std::filesystem;
  std::string path = "./textures";

  try {
    if (fs::exists(path) && fs::is_directory(path)) {
      for (const auto &entry : fs::directory_iterator(path)) {
        if (fs::is_regular_file(entry.path())) {
          texture_manager[entry.path().stem()] = std::make_shared<Texture>(entry.path());
          texture_names.push_back(entry.path().stem());
        }
      }
    }
  } catch (const fs::filesystem_error &e) {
    std::cerr << "Error: " << e.what() << "\n";
  }
}

App::~App() {
  glfwTerminate();
}
