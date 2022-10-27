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

uniform vec2 u_resolution;

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

void main() {
    vec4 proj_pos = u_projection * u_view * u_model * vec4(position, 1.0);
    // Polygon jittering
    vec4 snapped_pos = snap(proj_pos);

    vec3 v_pos = vec3(u_view * u_model * vec4(position, 1.0));
    vec3 v_normal = transpose(inverse(mat3(u_view * u_model))) * normal;

    vec3 lighting_dir = normalize(u_light - v_pos);
    vec3 light_color = vec3(1.0, 1.0, 1.0);
    vec3 light_ambient = vec3(0.1, 0.1, 0.1);

    float lightMag = max(dot(lighting_dir, v_normal), 0.0);
    vec3 diffuse = lightMag * light_color;

    v_color = vec4(color.xyz * (light_ambient + diffuse), color.w);
    v_uv = uv;

    gl_Position = snapped_pos;
}