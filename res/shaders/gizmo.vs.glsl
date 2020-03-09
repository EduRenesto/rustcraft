#version 430 core

layout(location=0) in vec3 in_Pos;
layout(location=1) in vec3 in_Normal;

layout(location=0) out vec3 fs_Normal;

uniform mat4 _RotX;
uniform mat4 _RotY;

void main() {
    fs_Normal = in_Normal;

    mat4 mat = _RotY * _RotX;

    gl_Position = mat * vec4(in_Pos, 1.0);
    gl_Position.z = -0.2;
}
