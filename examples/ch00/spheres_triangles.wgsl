struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) scene_position: vec4<f32>,
    @location(1) scene_normal: vec4<f32>,
    @location(2) color: vec4<f32>
}

fn norm(n: vec4f) -> vec4f {
    return vec4f(normalize(n.xyz), 0.0);
}

@vertex
fn vs_main(in: VertexNC) -> Output {
    let scene_position = model_u.points * in.position;

    var output: Output;
    output.position = camera_u.project * camera_u.view * scene_position;
    output.scene_position = scene_position;
    output.scene_normal = norm(model_u.normals * in.normal);
    output.color = in.color;
    return output;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
//    return vec4(in.scene_normal.y / 2.0 + 0.5, 0.0, 0.0, 1.0);
    return color(in.scene_position, in.scene_normal, in.color.rgb);
}
