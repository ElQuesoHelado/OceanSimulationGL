#include "Shader.hpp"

#include <fstream>
#include <iostream>
#include <print>
#include <sstream>

#include <glm/gtc/type_ptr.hpp>

std::string Shader::readFile(const std::string &path) {
  std::ifstream file(path);

  if (!file.is_open()) {
    throw std::runtime_error("Failed to open shader file: " + path);
  }

  std::stringstream ss;
  ss << file.rdbuf();

  return ss.str();
}

GLuint Shader::compileShader(GLenum type,
                             const std::string &source) {
  GLuint shader = glCreateShader(type);

  const char *src = source.c_str();

  glShaderSource(shader, 1, &src, nullptr);
  glCompileShader(shader);

  GLint success;
  glGetShaderiv(shader, GL_COMPILE_STATUS, &success);

  if (!success) {
    char info[512];

    glGetShaderInfoLog(shader, 512, nullptr, info);

    std::cerr << "Shader compilation error:\n"
              << info << std::endl;
  }

  return shader;
}

Shader::Shader(const std::string &vertexPath,
               const std::string &fragmentPath) {

  std::string vertSrc = readFile(vertexPath);
  std::string fragSrc = readFile(fragmentPath);

  // std::println("{}", vertSrc);

  GLuint vert = compileShader(GL_VERTEX_SHADER, vertSrc);
  GLuint frag = compileShader(GL_FRAGMENT_SHADER, fragSrc);

  ID = glCreateProgram();

  glAttachShader(ID, vert);
  glAttachShader(ID, frag);

  glLinkProgram(ID);

  GLint success;
  glGetProgramiv(ID, GL_LINK_STATUS, &success);

  if (!success) {
    char info[512];

    glGetProgramInfoLog(ID, 512, nullptr, info);

    std::cerr << "Program link error:\n"
              << info << std::endl;
  }

  glDeleteShader(vert);
  glDeleteShader(frag);
}

Shader::~Shader() {
  glDeleteProgram(ID);
}

void Shader::use() const {
  glUseProgram(ID);
}

GLuint Shader::getID() const {
  return ID;
}

void Shader::setBool(const std::string &name,
                     bool value) const {
  glUniform1i(
      glGetUniformLocation(ID, name.c_str()),
      (int)value);
}

void Shader::setInt(const std::string &name,
                    int value) const {
  glUniform1i(
      glGetUniformLocation(ID, name.c_str()),
      value);
}

void Shader::setFloat(const std::string &name,
                      float value) const {
  glUniform1f(
      glGetUniformLocation(ID, name.c_str()),
      value);
}

void Shader::setVec3(const std::string &name,
                     const glm::vec3 &value) const {
  glUniform3fv(
      glGetUniformLocation(ID, name.c_str()),
      1,
      glm::value_ptr(value));
}

void Shader::setMat4(const std::string &name,
                     const glm::mat4 &mat) const {
  glUniformMatrix4fv(
      glGetUniformLocation(ID, name.c_str()),
      1,
      GL_FALSE,
      glm::value_ptr(mat));
}
