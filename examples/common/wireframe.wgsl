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

struct TwoSideLightAux {
    is_two_side: i32,
}

@fragment
fn fs_main(@location(0) v_color: vec4<f32>) -> @location(0) vec4<f32> {
    return v_color;
}
