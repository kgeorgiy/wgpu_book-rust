struct VertexUniforms {
    model: mat4x4<f32>,
    model_it: mat4x4<f32>,
    view_project: mat4x4<f32>,
};
@binding(0) @group(0) var<uniform> vertex_u: VertexUniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_color: vec4<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec4<f32>, @location(1) normal: vec4<f32>) -> Output {
    var output: Output;
    output.position = vertex_u.view_project * vertex_u.model * pos;
    if (length(normal) == 0.0) {
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
