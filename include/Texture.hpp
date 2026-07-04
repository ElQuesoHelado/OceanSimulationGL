#pragma once

#include <glad/glad.h>
#include <memory>
#include <print>
#include <stb_image.h>

struct Texture {
  GLuint id{};

  int width, height, channels;
  std::string path;

  std::unique_ptr<unsigned char, decltype(&stbi_image_free)> data;

  Texture(std::string path)
      : data(
            stbi_load(path.c_str(), &width, &height, &channels, 0),
            stbi_image_free),
        path(path) {
    upload(); // TODO: idealmente llamado por alguien mas
  }

  void upload() {
    if (id != 0)
      return;

    glCreateTextures(GL_TEXTURE_2D, 1, &id);

    glTextureParameteri(id, GL_TEXTURE_WRAP_S, GL_REPEAT);
    glTextureParameteri(id, GL_TEXTURE_WRAP_T, GL_REPEAT);

    glTextureParameteri(id,
                        GL_TEXTURE_MIN_FILTER,
                        GL_LINEAR);

    glTextureParameteri(id,
                        GL_TEXTURE_MAG_FILTER,
                        GL_LINEAR);

    GLenum internalFormat;
    GLenum format;

    switch (channels) {
    case 1:
      internalFormat = GL_R8;
      format = GL_RED;
      break;

    case 3:
      internalFormat = GL_RGB8;
      format = GL_RGB;
      break;

    case 4:
      internalFormat = GL_RGBA8;
      format = GL_RGBA;
      break;

    default:
      throw std::runtime_error("Unsupported texture format");
    }

    glTextureStorage2D(id,
                       1,
                       internalFormat,
                       width,
                       height);

    glTextureSubImage2D(id,
                        0,
                        0,
                        0,
                        width,
                        height,
                        format,
                        GL_UNSIGNED_BYTE,
                        data.get());

    data.reset();
  }

  ~Texture() {
    if (id)
      glDeleteTextures(1, &id);
  }
};
