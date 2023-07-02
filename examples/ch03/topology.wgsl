struct Output {
    @location(0) v_color: vec4<f32>,
    @builtin(position) position: vec4<f32>,
}

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> Output {
    var pos = array<vec2<f32>,6>(
        vec2<f32>(-0.5, 0.7),
        vec2<f32>( 0.3, 0.6),
        vec2<f32>( 0.5, 0.3),
        vec2<f32>( 0.4, -0.5),
        vec2<f32>(-0.4, -0.4),
        vec2<f32>(-0.3, 0.2)
    );
    var color : array<vec3<f32>, 6> = array<vec3<f32>, 6>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0),
        vec3<f32>(1.0, 1.0, 0.0),
        vec3<f32>(0.0, 1.0, 1.0),
        vec3<f32>(1.0, 0.0, 1.0),
    );

    var out: Output;
    out.position = vec4<f32>(pos[vertex_index], 0.0, 1.0);
    out.v_color = vec4<f32>(color[vertex_index], 1.0);
    return out;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return in.v_color;
}
