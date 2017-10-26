#[test]
fn ready() {
    println!("it works!")
}

use svg::node::element::{Path, SVG};
use shape_triangulation::{triangulate_2d_f32, Triangulation};
use shape_triangulation::utils::{random32_in_ellipse, random32_in_rectangle, random64_in_ellipse, TriangulationSVG};

#[test]
fn test() {
    let renderer = TriangulationSVG::default();
    let points = random32_in_rectangle(1000.0, 618.0, 100);
    let result = triangulate_2d_f32(&points);
    let rect = result.get_area();
    let svg = renderer.render(&result);
    svg::save("image.svg", &svg).unwrap();
}
