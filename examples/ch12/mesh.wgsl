struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexC) -> Output {
    var output: Output;
    output.position = camera_u.view_project * model_u.points * in.position;
    output.v_color = in.color;
    return output;
}

struct TwoSideLightAux {
    is_two_side: i32,
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return in.v_color;
}
