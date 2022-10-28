#version 140

noperspective in vec4 v_color;
noperspective in vec2 v_uv;
in float v_fog_density;

out vec4 color;

uniform sampler2D u_texture;

void main() {
    vec4 fog_color = vec4(0.3, 0.3, 0.3, 1.0);
    // with texture
    color = mix(v_color * texture(u_texture, vec2(v_uv.x, 1.0 - v_uv.y)), fog_color, v_fog_density);
    // without texture
    //color = v_color;
}