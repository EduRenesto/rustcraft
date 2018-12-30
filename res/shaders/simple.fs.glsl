#version 430 core

in vec4 out_Pos;
in vec3 out_Norm;
in vec2 out_Uv;

out vec4 out_Color;

uniform sampler2D _Text;

void main() {
    //out_Color = vec4(1.0, 0.0, 1.0, 1.0);
    out_Color = texture2D(_Text, out_Uv);
}
