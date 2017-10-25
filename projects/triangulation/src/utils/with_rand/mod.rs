use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use shape_core::{Point};


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

pub fn random64_in_circle(radius: f64, points: usize) -> Vec<Point<f64>> {
    let mut rand = SmallRng::from_entropy();
    let mut out = Vec::with_capacity(points);
    for _ in 0..points {
        let angle = rand.gen_range(0.0..std::f64::consts::PI * 2.0);
        let distance = rand.gen_range(0.0..radius);
        out.push(Point::new(angle.cos() * distance, angle.sin() * distance));
    }
    out
}


pub fn random32_in_circle(radius: f32, points: usize) -> Vec<Point<f32>> {
    let mut rand = SmallRng::from_entropy();
    let mut out = Vec::with_capacity(points);
    for _ in 0..points {
        let angle = rand.gen_range(0.0..std::f32::consts::PI * 2.0);
        let distance = rand.gen_range(0.0..radius);
        out.push(Point::new(angle.cos() * distance, angle.sin() * distance));
    }
    out
}