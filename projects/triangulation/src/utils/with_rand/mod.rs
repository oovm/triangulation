use rand::{rngs::SmallRng, Rng, SeedableRng};
use shape_core::Point;

pub fn random64_in_rectangle(width: f64, height: f64, points: usize) -> Vec<Point<f64>> {
    let mut rand = SmallRng::from_entropy();
    let mut out = Vec::with_capacity(points);
    for _ in 0..points {
        out.push(Point::new(rand.gen_range(0.0..width), rand.gen_range(0.0..height)));
    }
    out
}

pub fn random32_in_rectangle(width: f32, height: f32, points: usize) -> Vec<Point<f32>> {
    let mut rand = SmallRng::from_entropy();
    let mut out = Vec::with_capacity(points);
    for _ in 0..points {
        out.push(Point::new(rand.gen_range(0.0..width), rand.gen_range(0.0..height)));
    }
    out
}

pub fn random64_in_ellipse(width: f64, height: f64, points: usize) -> Vec<Point<f64>> {
    let mut rand = SmallRng::from_entropy();
    let mut out = Vec::with_capacity(points);
    for _ in 0..points {
        let phi = rand.gen_range(0.0..std::f64::consts::PI * 2.0);
        let rho = rand.gen_range(0.0..1.0f64);
        out.push(Point::new(rho.sqrt() * width * phi.cos() / 2.0, rho.sqrt() * height * phi.sin() / 2.0));
    }
    out
}

pub fn random32_in_ellipse(width: f32, height: f32, points: usize) -> Vec<Point<f32>> {
    let mut rand = SmallRng::from_entropy();
    let mut out = Vec::with_capacity(points);
    for _ in 0..points {
        let phi = rand.gen_range(0.0..std::f32::consts::PI * 2.0);
        let rho = rand.gen_range(0.0..1.0f32);
        out.push(Point::new(rho.sqrt() * width * phi.cos() / 2.0, rho.sqrt() * height * phi.sin() / 2.0));
    }
    out
}
