#version 450

layout(std140, set = 0, binding = 0) uniform CustomUniformArgs {
    uniform float scale;
    uniform float iTime;
};


layout(location = 0) in vec2 pos;
layout(location = 1) in vec4 color;

layout(location = 0) out VertexData {
    vec2 pos;
    vec4 color;
} vertex;
layout(location = 2) out float time;


void main() {

    vertex.pos = pos;
    vertex.color = color;
    time = iTime;
    vec4 position = vec4(pos*scale, 0.0, 1.0);
    gl_Position = position;
}