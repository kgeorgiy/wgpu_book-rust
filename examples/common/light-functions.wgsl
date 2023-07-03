fn diffuse(dotNL: f32) -> f32 {
    return light_u.diffuse_intensity * max(dotNL, 0.0);
}

fn specular(dotNH: f32) -> f32 {
    return light_u.specular_intensity * pow(max(dotNH, 0.0), light_u.specular_shininess);
}

fn color(position: vec4<f32>, normal: vec4<f32>, color: vec3<f32>) -> vec4<f32> {
    return color_both(position, normal, color, 0.0);
}

fn color_both(position: vec4<f32>, normal: vec4<f32>, color: vec3<f32>, back: f32) -> vec4<f32> {
    let N: vec3<f32> = normalize(normal.xyz);
    let L: vec3<f32> = normalize(light_u.position.xyz - position.xyz);
    let V: vec3<f32> = normalize(camera_u.eye.xyz - position.xyz);
    let H: vec3<f32> = normalize(L + V);
    let dotNL = dot(N, L);
    let dotNH = dot(N, H);


    var diffuse: f32 = diffuse(dotNL);
    var specular: f32 = specular(dotNH);

    if (back != 0.0) {
        diffuse += back * diffuse(-dotNL);
        specular += back * specular(-dotNH);
    }

    let ambient = light_u.ambient_intensity;
    return vec4(color * (ambient + diffuse) + light_u.specular_color.xyz * specular, 1.0);
}
