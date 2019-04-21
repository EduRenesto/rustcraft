#version 430 core

layout(location=0) in vec3 in_Pos;
layout(location=2) in vec2 in_Uv;

layout(location=0) out vec2 out_Uv;

void main() {
    out_Uv = in_Uv;
    gl_Position = vec4(in_Pos, 1.0);
}
