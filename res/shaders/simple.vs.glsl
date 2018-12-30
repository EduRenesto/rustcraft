#version 430 core

layout(location=0) in vec3 in_Pos;
layout(location=1) in vec3 in_Norm;
layout(location=2) in vec2 in_Uv;

out vec4 out_Pos;
out vec3 out_Norm;
out vec2 out_Uv;

uniform mat4 _Projection;
uniform mat4 _View;

void main() {
    mat4 mvp = _Projection * _View;

    out_Pos = vec4(in_Pos, 1.0);
    out_Norm = in_Norm;
    out_Uv = in_Uv;

    gl_Position = mvp * out_Pos;
}
