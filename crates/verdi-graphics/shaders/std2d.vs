#version 150

// in
in vec3 position;
in vec3 normal;
in vec4 color;
in vec2 uv;

// out
out vec4 v_color;
out vec2 v_uv;

// matrices
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

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
    vec4 proj_vertex = u_projection * view_vertex;

    // Polygon jittering
    vec4 snapped_pos = snap(proj_vertex);

    gl_Position = snapped_pos;

    // final color
    v_color = vec4(color.xyz * (ambient_comp + diffuse_comp), color.w);
    v_uv = uv;
}