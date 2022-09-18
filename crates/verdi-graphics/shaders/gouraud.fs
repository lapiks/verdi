#version 140

in vec3 v_normal;
in vec4 v_color;
in vec2 v_uv;

out vec4 color;

uniform vec3 u_light;

uniform sampler2D tex;

void main() {
    color = texture(tex, v_uv);
}