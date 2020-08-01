#version 430 core

layout(location=0) in vec2 out_Uv;

uniform sampler2D _Albedo;
uniform sampler2D _Normal;
uniform sampler2D _Position;
uniform sampler2D _Occlusion;

out vec4 out_Color;

void main() {
    vec3 lightPos = vec3(-20, 10, 20);
    vec4 lightColor = vec4(1.0, 1.0, 1.0, 1.0);

    vec4 ambient = vec4(0.3, 0.3, 0.3, 1.0);
    vec4 color = texture2D(_Albedo, out_Uv);
    float occlusion = texture2D(_Occlusion, out_Uv).x;

    vec3 normal = texture2D(_Normal, out_Uv).xyz;
    vec3 position = texture2D(_Position, out_Uv).xyz;

    vec3 lightDir = normalize(lightPos - position);
    vec4 diffuse = max(dot(normalize(normal), lightDir), 0.0) * lightColor;

    //out_Color = occlusion * diffuse * color; 
    out_Color = color * ((ambient * occlusion) + diffuse);
    //out_Color = color * ((ambient) + diffuse);
}
