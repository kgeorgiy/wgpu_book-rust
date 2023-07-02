struct Uniforms {
    mvpMatrix: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}


@vertex
fn vs_main(in: VertexC) -> Output {
    var output: Output;
    output.position = uniforms.mvpMatrix * in.position;
    output.color = in.color;
    return output;
}

@fragment
fn fs_main(@location(0) color: vec4<f32>) -> @location(0) vec4<f32> {
    return color;
}
