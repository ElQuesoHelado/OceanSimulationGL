cmake_minimum_required(VERSION 3.5)

project(paint)

set(CMAKE_CXX_STANDARD 23)

find_package(glfw3 CONFIG QUIET)
if(NOT glfw3_FOUND)
  find_package(glfw3 REQUIRED)
endif()

find_package(glm CONFIG QUIET)
if(NOT glm_FOUND)
  find_package(glm REQUIRED)
endif()

find_package(OpenGL REQUIRED)

# --- NUEVO: Rutas y archivos fuente de ImGui ---
set(IMGUI_DIR ${CMAKE_CURRENT_SOURCE_DIR}/imgui)
set(IMGUI_SOURCES
    ${IMGUI_DIR}/imgui.cpp
    ${IMGUI_DIR}/imgui_draw.cpp
    ${IMGUI_DIR}/imgui_tables.cpp
    ${IMGUI_DIR}/imgui_widgets.cpp
    ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
    ${IMGUI_DIR}/backends/imgui_impl_opengl3.cpp
)
# -----------------------------------------------

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
    ${IMGUI_SOURCES}
)

target_include_directories(app PRIVATE
        glad/include
        include
        ${IMGUI_DIR}           # <-- Para que encuentre <imgui.h>
        ${IMGUI_DIR}/backends  # <-- Para que encuentre las integraciones (glfw/opengl)
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
