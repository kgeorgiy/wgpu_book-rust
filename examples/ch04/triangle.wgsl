struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

@vertex
fn vs_main(in: Vertex) -> Output {
    var
    out: Output;
    out.color = vec4<f32>(in.color, 1.0);
    out.position = vec4<f32>(in.position, 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return in.color;
} 
