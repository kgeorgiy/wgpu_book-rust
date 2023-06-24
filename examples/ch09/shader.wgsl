struct VertexUniforms {
    model: mat4x4<f32>,
    model_it: mat4x4<f32>,
    view: mat4x4<f32>,
};

@binding(0) @group(0) var<uniform> vertex_u: VertexUniforms;

struct Input {
    @location(0) pos: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) color: vec4<f32>,
};

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_color: vec4<f32>,
};

@vertex
fn vs_main(in: Input) -> Output {
    var output: Output;
    let position: vec4<f32> = vertex_u.model * in.pos;
    output.position = vertex_u.view * position;
    output.v_position = position;
    output.v_normal = vertex_u.model_it * in.normal;
    output.v_color = in.color;
    return output;
}


struct FragmentUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
};
@binding(1) @group(0) var<uniform> fragment_u: FragmentUniforms;

struct LightUniforms {
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
    is_two_side: i32,
};
@binding(2) @group(0) var<uniform> light_u: LightUniforms;

@fragment
fn fs_main(in: Output) -> @location(0) vec4<f32> {
    let N: vec3<f32> = normalize(in.v_normal.xyz);
    let L: vec3<f32> = normalize(fragment_u.light_position.xyz - in.v_position.xyz);
    let V: vec3<f32> = normalize(fragment_u.eye_position.xyz - in.v_position.xyz);
    let H: vec3<f32> = normalize(L + V);

    // front side
    var diffuse: f32 = light_u.diffuse_intensity * max(dot(N, L), 0.0);
    var specular: f32 = light_u.specular_intensity * pow(max(dot(N, H),0.0), light_u.specular_shininess);

    // back side
    if (light_u.is_two_side != 0) {
        diffuse = diffuse + light_u.diffuse_intensity * max(dot(-N, L), 0.0);
        specular = specular + light_u.specular_intensity * pow(max(dot(-N, H),0.0), light_u.specular_shininess);
    }

    let ambient: f32 = light_u.ambient_intensity;
    return vec4<f32>(in.v_color.xyz * (ambient + diffuse) + light_u.specular_color.xyz * specular, 1.0);
}
