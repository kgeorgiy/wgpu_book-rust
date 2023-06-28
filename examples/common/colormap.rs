#![allow(clippy::module_name_repetitions)]

use cgmath::Point3;

// Colormap

#[derive(Clone)]
#[allow(dead_code)]
pub struct Colormap {
    colors: Vec<Point3<f32>>,
}

#[allow(dead_code)]
impl Colormap {
    #[must_use] pub fn new(colors: &[[f32; 3]]) -> Self {
        Self { colors: colors.iter().map(|color| Point3::from(*color)).collect() }
    }

    #[must_use] pub fn by_name(name: &str) -> Self {
        let colormap = match name {
            "hsv" => [[1.0, 0.0, 0.0], [1.0, 0.5, 0.0], [0.97, 1.0, 0.01], [0.0, 0.99, 0.04], [0.0, 0.98, 0.52], [0.0, 0.98, 1.0], [0.01, 0.49, 1.0], [0.03, 0.0, 0.99], [1.0, 0.0, 0.96], [1.0, 0.0, 0.49], [1.0, 0.0, 0.02]],
            "hot" => [[0.0, 0.0, 0.0], [0.3, 0.0, 0.0], [0.6, 0.0, 0.0], [0.9, 0.0, 0.0], [0.93, 0.27, 0.0], [0.97, 0.55, 0.0], [1.0, 0.82, 0.0], [1.0, 0.87, 0.25], [1.0, 0.91, 0.5], [1.0, 0.96, 0.75], [1.0, 1.0, 1.0]],
            "cool" => [[0.49, 0.0, 0.7], [0.45, 0.0, 0.85], [0.42, 0.15, 0.89], [0.38, 0.29, 0.93], [0.27, 0.57, 0.91], [0.0, 0.8, 0.77], [0.0, 0.97, 0.57], [0.0, 0.98, 0.46], [0.0, 1.0, 0.35], [0.16, 1.0, 0.03], [0.58, 1.0, 0.0]],
            "spring" => [[1.0, 0.0, 1.0], [1.0, 0.1, 0.9], [1.0, 0.2, 0.8], [1.0, 0.3, 0.7], [1.0, 0.4, 0.6], [1.0, 0.5, 0.5], [1.0, 0.6, 0.4], [1.0, 0.7, 0.3], [1.0, 0.8, 0.2], [1.0, 0.9, 0.1], [1.0, 1.0, 0.0]],
            "summer" => [[0.0, 0.5, 0.4], [0.1, 0.55, 0.4], [0.2, 0.6, 0.4], [0.3, 0.65, 0.4], [0.4, 0.7, 0.4], [0.5, 0.75, 0.4], [0.6, 0.8, 0.4], [0.7, 0.85, 0.4], [0.8, 0.9, 0.4], [0.9, 0.95, 0.4], [1.0, 1.0, 0.4]],
            "autumn" => [[1.0, 0.0, 0.0], [1.0, 0.1, 0.0], [1.0, 0.2, 0.0], [1.0, 0.3, 0.0], [1.0, 0.4, 0.0], [1.0, 0.5, 0.0], [1.0, 0.6, 0.0], [1.0, 0.7, 0.0], [1.0, 0.8, 0.0], [1.0, 0.9, 0.0], [1.0, 1.0, 0.0]],
            "winter" => [[0.0, 0.0, 1.0], [0.0, 0.1, 0.95], [0.0, 0.2, 0.9], [0.0, 0.3, 0.85], [0.0, 0.4, 0.8], [0.0, 0.5, 0.75], [0.0, 0.6, 0.7], [0.0, 0.7, 0.65], [0.0, 0.8, 0.6], [0.0, 0.9, 0.55], [0.0, 1.0, 0.5]],
            "bone" => [[0.0, 0.0, 0.0], [0.08, 0.08, 0.11], [0.16, 0.16, 0.23], [0.25, 0.25, 0.34], [0.33, 0.33, 0.45], [0.41, 0.44, 0.54], [0.5, 0.56, 0.62], [0.58, 0.67, 0.7], [0.66, 0.78, 0.78], [0.83, 0.89, 0.89], [1.0, 1.0, 1.0]],
            "cooper" => [[0.0, 0.0, 0.0], [0.13, 0.08, 0.05], [0.25, 0.16, 0.1], [0.38, 0.24, 0.15], [0.5, 0.31, 0.2], [0.62, 0.39, 0.25], [0.75, 0.47, 0.3], [0.87, 0.55, 0.35], [1.0, 0.63, 0.4], [1.0, 0.71, 0.45], [1.0, 0.78, 0.5]],
            "greys" => [[0.0, 0.0, 0.0], [0.1, 0.1, 0.1], [0.2, 0.2, 0.2], [0.3, 0.3, 0.3], [0.4, 0.4, 0.4], [0.5, 0.5, 0.5], [0.6, 0.6, 0.6], [0.7, 0.7, 0.7], [0.8, 0.8, 0.8], [0.9, 0.9, 0.9], [1.0, 1.0, 1.0]],
            // "jet" as default
            _ => [[0.0, 0.0, 0.51], [0.0, 0.24, 0.67], [0.01, 0.49, 0.78], [0.01, 0.75, 0.89], [0.02, 1.0, 1.0], [0.51, 1.0, 0.5], [1.0, 1.0, 0.0], [0.99, 0.67, 0.0], [0.99, 0.33, 0.0], [0.98, 0.0, 0.0], [0.5, 0.0, 0.0]],
        };
        Self::new(&colormap)
    }

    #[must_use] pub fn interpolator(&self, min_max: (f32, f32)) -> ColormapInterpolator {
        ColormapInterpolator { colormap: self, min_max }
    }

    fn interpolate(&self, value: f32, (min, max): (f32, f32)) -> Point3<f32> {
        #![allow(clippy::indexing_slicing)]
        let tn = (value.clamp(min, max) - min) / (max - min);
        let len1 = self.colors.len() as f32 - 1.0;
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        let index = (len1 * tn).floor() as usize;

        if index == self.colors.len() - 1 {
            self.colors[index]
        } else {
            let a = self.colors[index];
            let b = self.colors[index + 1];
            a + (b - a) * (tn * len1 - index as f32)
        }
    }

    pub(crate) fn fixed(color: Point3<f32>) -> Self {
        Self::new(&[color.into()])
    }
}

// ColorInterpolator

#[allow(dead_code)]
pub struct ColormapInterpolator<'a> {
    colormap: &'a Colormap,
    min_max: (f32, f32),
}

#[allow(dead_code)]
impl<'a> ColormapInterpolator<'a> {
    #[must_use] pub fn interpolate(&self, value: f32) -> Point3<f32> {
        self.colormap.interpolate(value, self.min_max)
    }
}
