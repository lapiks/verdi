#version 140

noperspective in vec4 v_color;
noperspective in vec2 v_uv;

out vec4 color;

void main() {
    color = v_color;
}