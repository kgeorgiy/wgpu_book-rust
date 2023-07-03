struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
}

@vertex
fn vs_main(in: VertexN) -> Output {
    let position: vec4<f32> = model_u.points * in.position;

    var output: Output;
    output.position = camera_u.view_project * position;
    output.v_position = position;
    output.v_normal = model_u.normals * in.normal;
    return output;
}

@fragment
fn fs_main(@location(0) v_position: vec4<f32>, @location(1) v_normal: vec4<f32>) -> @location(0) vec4<f32> {
    return color(v_position, v_normal, color_light_u.color.xyz);
}
