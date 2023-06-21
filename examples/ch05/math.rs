use std::f64::consts::PI;
use cgmath::{Matrix4, Rad, Vector3, Vector4};

fn main() {
    show(
        "scaling",
        Matrix4::from_nonuniform_scale(0.5, 0.5, 1.5),
        Matrix4::from_nonuniform_scale(0.5, 0.5, 1.5) * Matrix4::from_scale(2.0)
    );
    show(
        "translation",
        Matrix4::from_translation(Vector3::new(3.0, 2.0, 1.0)),
        Matrix4::from_translation(Vector3::new(1.0, 2.0, 3.0)),
    );
    show(
        "rotation",
        Matrix4::from_angle_z(Rad(PI / 2.0)),
        Matrix4::from_angle_z(Rad(PI / 2.0)),
    );
}

fn show(operation: &str, first: Matrix4<f64>, second: Matrix4<f64>) {
    println!("\n{}", operation);

    let my_vec = Vector4::new(1.0, 2.0, 3.0, 1.0);
    println!("    Original vector: {:?}", my_vec);

    println!("    {} matrix: {:?}", operation, first);
    println!("    vector after {}: {:?}", operation, first * my_vec);

    let double = second * first;
    println!("    Double {} matrix: {:?}", operation, double);
    println!("    Vector after double {}: {:?}", operation, double * my_vec);
}
