#version 430 core

in vec4 out_Pos;
in vec3 out_Norm;
in vec2 out_Uv;

out vec4 out_Color;

uniform sampler2D _Text;

void main() {
    vec3 lightPos = vec3(20.0, 4.0, 0.0);
    vec4 lightColor = vec4(1.0, 1.0, 1.0, 1.0);

    //out_Color = vec4(1.0, 0.0, 1.0, 1.0);
    vec4 albedo = texture2D(_Text, out_Uv);

    vec3 lightDir = normalize(lightPos - out_Pos.xyz);
    vec4 diffuse = max(dot(out_Norm, lightDir), 0.0) * lightColor;

    vec4 result = diffuse * albedo;

    out_Color = result;
}
