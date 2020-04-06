#Compile vertex shader

gl-compile -V ./assets/shaders/glsl/vertex/triangle.vert -o ./assets/shaders/compiled/vertex/custom.vert.spv

#Compile fragment shader

gl-compile -V ./assets/shaders/glsl/fragment/colors.frag -o ./assets/shaders/compiled/fragment/custom.frag.spv

cargo run --release