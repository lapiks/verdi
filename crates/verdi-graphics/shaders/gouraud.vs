#version 150

in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 uv;

out vec3 v_normal;
out vec4 v_color;
out vec2 v_uv;

uniform mat4 matrix;

void main() {
    v_normal = transpose(inverse(mat3(matrix))) * normal;
    v_color = color;
    v_uv = uv;
    gl_Position = matrix * vec4(position, 1.0);
}