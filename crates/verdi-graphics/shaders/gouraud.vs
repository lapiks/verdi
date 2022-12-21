#version 150

// in
in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 uv;

// out
out vec4 v_color;
out vec2 v_uv;
out float v_fog_density;

// light
uniform bool u_enable_lighting;
uniform vec3 u_light;

// matrices
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

uniform vec2 u_resolution;

// fog
uniform bool u_enable_fog;
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
    vec4 world_vertex = u_model * vec4(position, 1.0);
    vec4 view_vertex = u_view * world_vertex;
    vec4 proj_vertex = u_projection * view_vertex;

    // Polygon jittering
    vec4 snapped_pos = snap(proj_vertex);

    gl_Position = snapped_pos;

    // fog
    v_fog_density = 0.0;
    if(u_enable_fog) {
        float vertex_depth = length(view_vertex);
        v_fog_density = clamp(inverse_lerp(u_fog_start, u_fog_end, vertex_depth), 0.0, 1.0);
    }

    // lighting
    if(u_enable_lighting) {
        const vec3 light_color = vec3(1.0, 1.0, 1.0);

        // ambient
        const float ambient_strength = 0.1;
        vec3 ambient_comp = ambient_strength * light_color;

        // diffuse
        vec3 v_normal = normalize(mat3(transpose(inverse(u_model))) * normal);
        vec3 lighting_dir = normalize(u_light - world_vertex.xyz);
        float light_mag = max(dot(lighting_dir, v_normal), 0.0);
        vec3 diffuse_comp = light_mag * light_color;

        // final color
        v_color = vec4(color.xyz * (ambient_comp + diffuse_comp), color.w);
    }
    else {
        v_color = color;
    }
    
    v_uv = uv;
}