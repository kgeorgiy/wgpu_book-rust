@vertex
fn vs_main(in: Vertex) -> @builtin(position) vec4<f32> {
    return camera_u.project * camera_u.view * model_u.points * in.position;
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4(1.0, 1.0, 0.0, 1.0);
}
