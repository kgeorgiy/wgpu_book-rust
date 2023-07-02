@vertex
fn vs_main(in: Vertex) -> @builtin(position) vec4<f32> {
    return mvp_u.matrix * in.position;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 1.0, 0.0, 1.0);
}

