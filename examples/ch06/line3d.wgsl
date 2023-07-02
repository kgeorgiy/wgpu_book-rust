struct Uniforms {
    mvp: mat4x4<f32>,
}

@group(0) @binding(0) var<uniform> uniforms : Uniforms;

@vertex
fn vs_main(in: Vertex) -> @builtin(position) vec4<f32> {
    return uniforms.mvp * in.position;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}

