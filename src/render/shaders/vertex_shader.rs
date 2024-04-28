pub const VERT_SHADER: &str = "
#version 460

layout(location = 0) in vec4 position;

layout(set = 0, binding = 0) uniform Transformations {
    mat4 world;
    mat4 view;
    mat4 projection;
    mat4 scale;
} transformations;

void main() {
    vec4 projection =
        transformations.projection*
        transformations.view*
        transformations.world*
        transformations.scale*
        position;

    gl_Position = projection;
}
";