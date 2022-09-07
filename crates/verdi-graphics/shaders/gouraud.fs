#version 140

in vec3 v_normal;
in vec4 v_color;
out vec4 color;
uniform vec3 u_light;

void main() {
    color = v_color;
}