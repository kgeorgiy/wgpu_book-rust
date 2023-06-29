struct ModelUniforms {
    points: mat4x4<f32>,
    normals: mat4x4<f32>,
}
@group(0) @binding(0) var<uniform> model_u: ModelUniforms;

struct CameraUniforms {
    view_project: mat4x4<f32>,
}
@group(0) @binding(1) var<uniform> camera_u: CameraUniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec4<f32>,
}

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) color: vec4<f32>) -> Output {
    var output: Output;
    output.position = camera_u.view_project * model_u.points * pos;
    output.v_color = color;
    return output;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return in.v_color;
}
