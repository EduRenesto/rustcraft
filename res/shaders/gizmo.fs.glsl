#version 430 core

layout(location=0) in vec3 fs_Normal;

out vec4 out_Color;

void main() {
    out_Color = vec4(fs_Normal, 1.0);
}
