#version 450
precision mediump float;
layout(location = 0) out vec4 outColor;
layout(location = 0) in float iTime;
layout(location = 1) in vec3 iResolution;
layout(location = 2) in vec2 fragCoord;

void main() {
    // Normalized pixel coordinates (from 0 to 1)
    vec2 uv = fragCoord/iResolution.xy;

    // Time varying pixel color
    vec3 col =  1.0* uv.xyx * cos(iTime*vec3(0,2,4));

    // Output to screen
    outColor = vec4(col,1.0);
}
