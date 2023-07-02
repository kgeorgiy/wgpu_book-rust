struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_color: vec4<f32>
}

@vertex
fn vs_main(in: VertexNC) -> Output {
    let position = model_u.points * in.position;

    var output: Output;
    output.position = camera_u.project * camera_u.view * position;
    output.v_position = position;
    output.v_normal = model_u.normals * in.normal;
    output.v_color = in.color;
    return output;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return color(in.v_position, in.v_normal, in.v_color.xyz);
}
