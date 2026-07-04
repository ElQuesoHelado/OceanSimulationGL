#pragma once

#include "MOps.hpp"
#include "Texture.hpp"
#include "par_shapes.h"
#include <cstdlib>
#include <glm/ext/vector_float2.hpp>
#include <glm/ext/vector_float3.hpp>
#include <iostream>
#include <memory>
#include <optional>
#include <print>
#include <variant>

#include <glad/glad.h>

struct Primitive {
  enum class Type {
    Cube,
    Sphere,
    Torus,
    Klein,
    Rock,
    Pen,
    Cylinder,
    Tetrahedron,
    Cone
  };

  Type type;
  GLuint VAO{}, VBO{}, normalsVBO{}, texCoordsVBO{}, EBO{};
  float shininess{128};
  std::shared_ptr<Texture> texture;

  par_shapes_mesh *shape;
  glm::vec3 pos{}, color{1};

  void createGLObjects() {
    glCreateVertexArrays(1, &VAO);
    glCreateBuffers(1, &VBO);
    glCreateBuffers(1, &normalsVBO);
    glCreateBuffers(1, &texCoordsVBO);
    glCreateBuffers(1, &EBO);
  }

  void setupVAO() {
    // posiciones
    glVertexArrayVertexBuffer(VAO, 0, VBO, 0, sizeof(glm::vec3));
    glEnableVertexArrayAttrib(VAO, 0);
    glVertexArrayAttribFormat(VAO, 0, 3, GL_FLOAT, GL_FALSE, 0);
    glVertexArrayAttribBinding(VAO, 0, 0);

    // normales
    glVertexArrayVertexBuffer(VAO, 1, normalsVBO, 0, sizeof(glm::vec3));
    glEnableVertexArrayAttrib(VAO, 1);
    glVertexArrayAttribFormat(VAO, 1, 3, GL_FLOAT, GL_FALSE, 0);
    glVertexArrayAttribBinding(VAO, 1, 1);

    // coordenadas UV
    glVertexArrayVertexBuffer(VAO, 2, texCoordsVBO, 0, sizeof(glm::vec2));
    glEnableVertexArrayAttrib(VAO, 2);
    glVertexArrayAttribFormat(VAO, 2, 2, GL_FLOAT, GL_FALSE, 0);
    glVertexArrayAttribBinding(VAO, 2, 2);
  }

  void initialize() {
    createGLObjects();
    setupVAO();
  }

  static Primitive Cube(glm::vec3 pos,
                        glm::vec3 color,
                        const std::shared_ptr<Texture> &tex) {
    auto cube = par_shapes_create_cube();
    par_shapes_unweld(cube, true);
    return {
        cube,
        Type::Cube,
        pos,
        color,
        tex};
  }

  static Primitive Sphere(glm::vec3 pos,
                          glm::vec3 color,
                          const std::shared_ptr<Texture> &tex) {
    return {
        par_shapes_create_parametric_sphere(15, 15),
        Type::Sphere,
        pos,
        color,
        tex};
  }

  static Primitive Torus(glm::vec3 pos,
                         glm::vec3 color,
                         const std::shared_ptr<Texture> &tex) {
    return {
        par_shapes_create_torus(15, 15, .5),
        Type::Torus,
        pos,
        color,
        tex};
  }

  static Primitive Klein(glm::vec3 pos,
                         glm::vec3 color,
                         const std::shared_ptr<Texture> &tex) {
    auto shape = par_shapes_create_klein_bottle(15, 15);
    par_shapes_scale(shape, .2, .2, .2);
    return {
        shape,
        Type::Klein,
        pos,
        color,
        tex};
  }

  static Primitive Rock(glm::vec3 pos,
                        glm::vec3 color,
                        const std::shared_ptr<Texture> &tex) {
    return {
        par_shapes_create_rock(rand() % 11, 2),
        Type::Rock,
        pos,
        color,
        tex};
  }

  static Primitive Cylinder(glm::vec3 pos,
                            glm::vec3 color,
                            const std::shared_ptr<Texture> &tex) {
    return {
        par_shapes_create_cylinder(20, 20),
        Type::Cylinder,
        pos,
        color,
        tex};
  }

  static Primitive Tetrahedron(glm::vec3 pos,
                               glm::vec3 color,
                               const std::shared_ptr<Texture> &tex) {
    return {
        par_shapes_create_tetrahedron(),
        Type::Tetrahedron,
        pos,
        color,
        tex};
  }

  static Primitive Pen(glm::vec3 pos,
                       glm::vec3 color,
                       const std::shared_ptr<Texture> &tex) {
    auto b1 =
        par_shapes_create_parametric_sphere(15, 15);
    par_shapes_translate(b1, 1, 0, 0);

    auto b2 =
        par_shapes_create_parametric_sphere(15, 15);
    par_shapes_translate(b2, -1, 0, 0);

    auto b3 =
        par_shapes_create_parametric_sphere(15, 15);
    par_shapes_translate(b3, 0, 1, 0);
    par_shapes_scale(b3, .7, 2, .7);

    auto pen = par_shapes_create_empty();
    par_shapes_merge_and_free(pen, b1);
    par_shapes_merge_and_free(pen, b2);
    par_shapes_merge_and_free(pen, b3);

    return {
        pen,
        Type::Pen,
        pos,
        color,
        tex};
  }

  static Primitive Cone(glm::vec3 pos,
                        glm::vec3 color,
                        const std::shared_ptr<Texture> &tex) {
    return {
        par_shapes_create_cone(15, 15),
        Type::Cone,
        pos,
        color,
        tex};
  }

  Primitive(par_shapes_mesh *_shape, Type type,
            glm::vec3 pos,
            glm::vec3 color, const std::shared_ptr<Texture> &ptr_tex)
      : shape(_shape),
        type(type),
        pos(pos),
        color(color),
        texture(ptr_tex) {
    par_shapes_translate(shape, pos.x, pos.y, pos.z);

    initialize();
    upload();
  }

  // Copy constructor: deep copy, buffers GL nuevos
  Primitive(const Primitive &other)
      : pos(other.pos),
        type(other.type),
        color(other.color),
        texture(other.texture) {
    shape = par_shapes_clone(other.shape, nullptr);

    initialize();
    upload();
  }

  Primitive &operator=(const Primitive &other) {
    if (this != &other) {
      destroy();

      pos = other.pos;
      type = other.type;
      color = other.color;
      shape = par_shapes_clone(other.shape, nullptr);
      texture = other.texture;

      initialize();
      upload();
    }

    return *this;
  }

  Primitive(Primitive &&other) noexcept
      : VAO(other.VAO),
        VBO(other.VBO),
        normalsVBO(other.normalsVBO),
        EBO(other.EBO),
        texture(std::move(other.texture)),
        shape(other.shape),
        pos(other.pos),
        type(other.type),
        color(other.color) {
    other.VAO = other.VBO = other.normalsVBO = other.EBO = 0;
    other.shape = nullptr;
  }

  Primitive &operator=(Primitive &&other) noexcept {
    if (this != &other) {
      destroy();

      VAO = other.VAO;
      VBO = other.VBO;
      normalsVBO = other.normalsVBO;
      EBO = other.EBO;
      shape = other.shape;
      texture = std::move(other.texture);
      pos = other.pos;
      color = other.color;
      type = other.type;

      other.VAO = other.VBO = other.normalsVBO = other.EBO = 0;
      other.shape = nullptr;
    }

    return *this;
  }

  void calc_cube_texcoords() {
    if (shape->tcoords)
      free(shape->tcoords);

    shape->tcoords =
        (float *)malloc(2 * shape->npoints * sizeof(float));

    for (int face = 0; face < 6; face++) {
      int v = face * 6;

      // triángulo 1
      shape->tcoords[2 * (v + 0)] = 1;
      shape->tcoords[2 * (v + 0) + 1] = 0;
      shape->tcoords[2 * (v + 1)] = 1;
      shape->tcoords[2 * (v + 1) + 1] = 1;
      shape->tcoords[2 * (v + 2)] = 0;
      shape->tcoords[2 * (v + 2) + 1] = 1;

      // triángulo 2
      shape->tcoords[2 * (v + 3)] = 0;
      shape->tcoords[2 * (v + 3) + 1] = 1;
      shape->tcoords[2 * (v + 4)] = 0;
      shape->tcoords[2 * (v + 4) + 1] = 0;
      shape->tcoords[2 * (v + 5)] = 1;
      shape->tcoords[2 * (v + 5) + 1] = 0;
    }
  }

  void calc_triplanar_texcoords() {
    if (shape->tcoords)
      free(shape->tcoords);
    shape->tcoords = (float *)malloc(2 * shape->npoints * sizeof(float));

    for (int i = 0; i < shape->npoints; i++) {
      float x = shape->points[3 * i + 0];
      float y = shape->points[3 * i + 1];
      float z = shape->points[3 * i + 2];

      float nx = fabsf(shape->normals[3 * i + 0]);
      float ny = fabsf(shape->normals[3 * i + 1]);
      float nz = fabsf(shape->normals[3 * i + 2]);

      float u, v;
      if (nx >= ny && nx >= nz) {
        u = z;
        v = y;
      } else if (ny >= nx && ny >= nz) {
        u = x;
        v = z;
      } else {
        u = x;
        v = y;
      }

      shape->tcoords[2 * i + 0] = u * 0.5f + 0.5f;
      shape->tcoords[2 * i + 1] = v * 0.5f + 0.5f;
    }
  }

  void upload() {
    par_shapes_compute_normals(shape);

    if (type == Type::Cube)
      calc_cube_texcoords();
    else if (!shape->tcoords) // TODO: ?Seguro?, ?uploads posteriores?
      calc_triplanar_texcoords();

    glNamedBufferData(
        VBO,
        shape->npoints * 3 * sizeof(float),
        shape->points,
        GL_STATIC_DRAW);

    glNamedBufferData(
        normalsVBO,
        shape->npoints * 3 * sizeof(float),
        shape->normals,
        GL_STATIC_DRAW);

    if (shape->tcoords) {
      glNamedBufferData(
          texCoordsVBO,
          shape->npoints * 2 * sizeof(float),
          shape->tcoords,
          GL_STATIC_DRAW);
    }

    glNamedBufferData(
        EBO,
        shape->ntriangles * 3 * sizeof(PAR_SHAPES_T),
        shape->triangles,
        GL_STATIC_DRAW);

    glVertexArrayElementBuffer(VAO, EBO);
    // std::cout << shape->normals << std::endl;
  }

  void destroy() {
    if (VAO)
      glDeleteVertexArrays(1, &VAO);
    if (VBO)
      glDeleteBuffers(1, &VBO);
    if (normalsVBO)
      glDeleteBuffers(1, &normalsVBO);
    if (texCoordsVBO)
      glDeleteBuffers(1, &texCoordsVBO);
    if (EBO)
      glDeleteBuffers(1, &EBO);

    if (shape)
      par_shapes_free_mesh(shape);

    VAO = VBO = normalsVBO = EBO = 0;
    shape = nullptr;
  }

  ~Primitive() {
    destroy();
  }

  void draw() const {
    glBindVertexArray(VAO);

    glBindTexture(GL_TEXTURE_2D, texture->id);

    // glDrawArrays(GL_POINTS, 0, shape->npoints);
    glDrawElements(
        GL_TRIANGLES,
        shape->ntriangles * 3,
        sizeof(PAR_SHAPES_T) == 2 ? GL_UNSIGNED_SHORT
                                  : GL_UNSIGNED_INT,
        nullptr);
  }

  void scale(glm::vec3 s) {
    par_shapes_translate(shape, -pos.x, -pos.y, -pos.z);
    par_shapes_scale(shape, s.x, s.y, s.z);
    par_shapes_translate(shape, pos.x, pos.y, pos.z);

    upload();
  }

  void translate(glm::vec3 s) {
    pos += s;
    par_shapes_translate(shape, s.x, s.y, s.z);
    upload();
  }

  void rotate(float ang, glm::vec3 ax, bool centered) {
    float a[3] = {ax.x, ax.y, ax.z};

    if (centered) {
      float neg[3] = {-pos.x, -pos.y, -pos.z};
      par_shapes_translate(shape, neg[0], neg[1], neg[2]);

      par_shapes_rotate(shape, ang, a);

      float fwd[3] = {pos.x, pos.y, pos.z};
      par_shapes_translate(shape, fwd[0], fwd[1], fwd[2]);

    } else {
      glm::mat4 m(1.0f);
      m = glm::rotate(m, ang, ax);
      pos = glm::vec3(m * glm::vec4(pos, 1.0f));
      par_shapes_rotate(shape, ang, a);
    }

    upload();
  }
};
