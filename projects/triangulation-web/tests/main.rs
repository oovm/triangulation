#[test]
fn ready() {
    println!("it works!")
}

use shape_triangulation::{triangulate_2d_f32};
use shape_triangulation::utils::{random32_in_rectangle, TriangulationSVG};

#[test]
fn test() {
    let renderer = TriangulationSVG::default();
    let points = random32_in_rectangle(1000.0, 618.0, 100);
    let result = triangulate_2d_f32(&points);
    let svg = renderer.render(&result);
    TriangulationSVG::save("image.svg", &svg).unwrap();
}
