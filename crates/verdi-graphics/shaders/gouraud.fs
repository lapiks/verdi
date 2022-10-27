#version 140

noperspective in vec4 v_color;
noperspective in vec2 v_uv;

out vec4 color;

uniform sampler2D u_texture;

void main() {
    color = v_color * texture(u_texture, vec2(v_uv.x, 1.0 - v_uv.y));
    //color = v_color * texture(u_texture, vec2(v_uv.x, 1.0 - v_uv.y));
    //color = v_color;
}