#[test]
fn ready() {
    println!("it works!")
}

use shape_triangulation::{
    triangulate_2d_f64,
    utils::{random32_in_ellipse, TriangulationSVG},
};

#[test]
fn test() {
    let renderer = TriangulationSVG::default();
    let points = random32_in_ellipse(1000.0, 618.0, 666);
    let result = triangulate_2d_f64(&points);
    let svg = renderer.render(&result);
    TriangulationSVG::save("image.svg", &svg).unwrap();
}
