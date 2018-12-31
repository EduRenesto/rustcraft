#version 430 core

layout(location=0) in vec2 out_Uv;

uniform sampler2D _Albedo;

out vec4 out_Color;

void main() {
    out_Color = texture2D(_Albedo, out_Uv);
}
