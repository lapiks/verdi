#version 140

noperspective in vec4 v_color;
in float v_fog_density;

out vec4 color;


void main() {
    vec4 fog_color = vec4(0.3, 0.3, 0.3, 1.0);

    // wo texture
    color = mix(v_color, fog_color, v_fog_density);
}