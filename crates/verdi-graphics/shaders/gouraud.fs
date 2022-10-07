#version 140

in vec4 v_color;
in vec2 v_uv;

out vec4 color;

uniform sampler2D u_texture;

void main() {
    color = texture(u_texture, vec2(v_uv.x, 1.0 - v_uv.y)); // y inversion ?
    //color = v_color;
}