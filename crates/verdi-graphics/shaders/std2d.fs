#version 140

noperspective in vec4 v_color;
noperspective in vec2 v_uv;

uniform sampler2D colorTexture;

out vec4 color;

void main() {
    //color = v_color;
    color = texture(colorTexture, v_uv);
}