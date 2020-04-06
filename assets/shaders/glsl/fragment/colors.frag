#version 450

layout(location = 0) in VertexData {
    vec2 pos;
    vec4 color;
    
} vertex;
layout(location = 2) in float iTime;

layout(location = 0) out vec4 out_color;

void main() {
    vec4 color = vertex.color;
    // out_color = vec4((vec3(vertex.pos,1.0) * cos(vertex.iTime*vec3(0,2,4))),1.0);
    out_color = vec4(0.5*cos(iTime*vec3(0,2,4)),1.0);
}