fn two_side_color(position: vec4<f32>, normal: vec4<f32>, color: vec3<f32>) -> vec4<f32> {
    var back: f32;
    if (two_side_light_u.is_two_side != 0) {
        back = 0.5;
    }
    return color_both(position, normal, color, back);
}
