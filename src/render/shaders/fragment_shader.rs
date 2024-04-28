pub const FRAG_SHADER: &str = "
#version 460

layout(location = 0) out vec4 color;

layout(set = 0, binding = 1) uniform Colordata {
    vec4 data[6];
} colordata;

void main(){
    color = colordata.data[gl_PrimitiveID/2];
}
";