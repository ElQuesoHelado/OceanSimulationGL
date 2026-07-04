cmake_minimum_required(VERSION 3.5)

project(paint)

set(CMAKE_CXX_STANDARD 23)

find_package(glfw3 REQUIRED)
find_package(OpenGL REQUIRED)
find_package(glm REQUIRED)
find_package(imgui REQUIRED)

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
)

target_include_directories(app PRIVATE
        glad/include
        include
        /usr/include/stb)

target_link_libraries(app glfw OpenGL::GL glm::glm imgui)

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
