struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_uv: vec2<f32>,
    @location(3) v_color: vec4<f32>,
}

@vertex
fn vs_main(in: VertexNCT) -> Output {
    let position: vec4<f32> = model_u.points * in.position;

    var output: Output;
    output.position = camera_u.view_project * position;
    output.v_position = position;
    output.v_normal = model_u.normals * in.normal;
    output.v_uv = in.uv;
    output.v_color = in.color;
    return output;
}

struct TwoSideLightAux {
    is_two_side: i32,
}

fn two_side_color(position: vec4<f32>, normal: vec4<f32>, color: vec3<f32>) -> vec4<f32> {
    var back: f32;
    if (light_u.aux.is_two_side != 0) {
        back = 0.5;
    }
    return color_both(position, normal, color, back);
}

@group(1) @binding(0) var texture_data: texture_2d<f32>;
@group(1) @binding(1) var texture_sampler: sampler;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    return color(in.v_position, in.v_normal, textureSample(texture_data, texture_sampler, in.v_uv).rgb + in.v_color.rgb);
}
