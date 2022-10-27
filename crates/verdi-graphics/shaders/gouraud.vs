#version 150

in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 uv;

out vec4 v_color;
out vec2 v_uv;
out float v_fog_density;

uniform vec3 u_light;

uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

uniform vec2 u_resolution;
uniform float u_fog_start;
uniform float u_fog_end;

// Polygon jittering
vec4 snap(vec4 vertex) {
    // convert to normalised device coordinates (NDC)
    vertex.xyz /= vertex.w; 
    // snap the vertex to the lower-resolution grid :
    // troncate in the target resolution and then get back to NDC
    vertex.xy = floor(u_resolution * vertex.xy) / u_resolution; 
    // get back to projection-space
    vertex.xyz *= vertex.w; 

    return vertex;
}

float inverse_lerp(float a, float b, float t) {
    return (t - a) / (b - a);
}

void main() {
    vec4 proj_pos = u_projection * u_view * u_model * vec4(position, 1.0);
    // Polygon jittering
    vec4 snapped_pos = snap(proj_pos);

    gl_Position = snapped_pos;

    // fog
    float vertex_depth = length(u_view * u_model * vec4(position, 1.0));
    v_fog_density = clamp(inverse_lerp(u_fog_start, u_fog_end, vertex_depth), 0.0, 1.0);

    vec3 v_pos = vec3(u_view * u_model * vec4(position, 1.0));
    vec3 v_normal = transpose(inverse(mat3(u_view * u_model))) * normal;

    vec3 lighting_dir = normalize(u_light - v_pos);
    vec3 light_color = vec3(1.0, 1.0, 1.0);
    vec3 light_ambient = vec3(0.1, 0.1, 0.1);

    float light_mag = max(dot(lighting_dir, v_normal), 0.0);
    vec3 diffuse = light_mag * light_color;

    v_color = vec4(color.xyz * (light_ambient + diffuse), color.w);
    v_uv = uv;
}