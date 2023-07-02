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
fn vs_main(in: VertexN) -> Output {
    var output: Output;
    output.position = camera_u.view_project * model_u.points * in.position;
    if (length(in.normal) == 0.0) {
        output.v_color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    } else {
        output.v_color = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }
    return output;
}

@fragment
fn fs_main(@location(0) v_color: vec4<f32>) -> @location(0) vec4<f32> {
    return v_color;
}
