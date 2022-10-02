#version 150

in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 uv;

out vec4 v_color;
out vec2 v_uv;

uniform vec3 u_light;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

void main() {
    vec3 v_pos = vec3(u_view * u_model * vec4(position, 1.0));
    vec3 v_normal = transpose(inverse(mat3(u_view * u_model))) * normal;

    vec3 lighting_dir = normalize(u_light - v_pos);
    float distance = length(u_light - v_pos);

    float attenuation = 1.0 / (1.0 + (0.25 * distance * distance));

    vec4 diffuse_color = vec4(0.6, 0.0, 0.0, 1.0);

    vec3 light_color = vec3(1.0, 1.0, 1.0);

    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;
    vec3 diffuse = max(dot(v_normal, lighting_dir), 0.0) * light_color;

    vec3 object_color = vec3(1.0, 0.0, 0.0);
    v_color = vec4((ambient + diffuse) * object_color, 1.0);
    v_uv = uv;

    gl_Position = u_projection * u_view * u_model * vec4(position, 1.0);
}