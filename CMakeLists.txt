cmake_minimum_required(VERSION 3.14) # Aumentado a 3.14 para compatibilidad con FetchContent

project(paint)

set(CMAKE_CXX_STANDARD 23)

# --- Descarga automática de ImGui ---
include(FetchContent)
FetchContent_Declare(
  imgui
  GIT_REPOSITORY https://github.com/ocornut/imgui.git
  GIT_TAG        v1.90.8
)
FetchContent_MakeAvailable(imgui)
# ------------------------------------

find_package(glfw3 CONFIG QUIET)
if(NOT glfw3_FOUND)
  find_package(glfw3 REQUIRED)
endif()

find_package(glm CONFIG QUIET)
if(NOT glm_FOUND)
  find_package(glm REQUIRED)
endif()

find_package(OpenGL REQUIRED)

add_executable(app
    main.cpp
    src/App/App.cpp
    src/App/config.cpp
    src/App/input.cpp
    src/App/projections.cpp
    src/App/ui.cpp
    src/Shader.cpp
    src/Mesh.cpp
    glad/src/glad.c
    src/par_shapes.cpp
    src/stb_image.cpp
    src/ocean/config.cpp
    src/ocean/draw.cpp
    # Archivos fuente de ImGui descargados automáticamente
    ${imgui_SOURCE_DIR}/imgui.cpp
    ${imgui_SOURCE_DIR}/imgui_draw.cpp
    ${imgui_SOURCE_DIR}/imgui_tables.cpp
    ${imgui_SOURCE_DIR}/imgui_widgets.cpp
    ${imgui_SOURCE_DIR}/backends/imgui_impl_glfw.cpp
    ${imgui_SOURCE_DIR}/backends/imgui_impl_opengl3.cpp
)

target_include_directories(app PRIVATE
        glad/include
        include
        ${imgui_SOURCE_DIR}
        ${imgui_SOURCE_DIR}/backends
)

target_link_libraries(app glfw OpenGL::GL glm::glm)

add_custom_command(
    TARGET app POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory $<TARGET_FILE_DIR:app>/shaders
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        ${CMAKE_CURRENT_SOURCE_DIR}/shaders
        $<TARGET_FILE_DIR:app>/shaders
)

add_custom_command(
    TARGET app POST_BUILD
    COMMAND ${CMAKE_COMMAND} -E make_directory $<TARGET_FILE_DIR:app>/textures
    COMMAND ${CMAKE_COMMAND} -E copy_directory
        ${CMAKE_CURRENT_SOURCE_DIR}/textures
        $<TARGET_FILE_DIR:app>/textures
)