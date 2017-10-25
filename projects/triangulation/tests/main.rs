use delaunay::triangulate_2d_f32;
use shape_core::Point;
use svg::node::element::SVG;

#[test]
fn test() {
    let points = vec![
        Point { x: 10.0, y: 70.0 },
        Point { x: 20.0, y: 80.0 },
        Point { x: 70.0, y: 90.0 },
        Point { x: 30.0, y: 60.0 },
        Point { x: 40.0, y: 50.0 },
        Point { x: 60.0, y: 40.0 },
        Point { x: 50.0, y: 10.0 },
        Point { x: 80.0, y: 30.0 },
        Point { x: 90.0, y: 20.0 },
    ];
    let result = triangulate_2d_f32(&points);
    let rect = result.get_area();
    let mut svg = SVG::new()
        .set("width", rect.side.0)
        .set("height", rect.side.1)
        .set("viewBox", (rect.anchor.x, rect.anchor.y, rect.side.0, rect.side.1));
    for [a, b, c] in result.triangle_vertexes() {
        svg = svg.add(
            svg::node::element::Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 1)
                .set("d", format!("M {},{} L {},{} L {},{} Z", a.x, a.y, b.x, b.y, c.x, c.y)),
        );
    }
    svg::save("image.svg", &svg).unwrap();
    assert_eq!(
        result.get_indexes(),
        vec![4, 3, 2, 3, 1, 2, 2, 5, 4, 6, 0, 3, 3, 0, 1, 2, 7, 5, 5, 6, 4, 4, 6, 3, 7, 6, 5, 2, 8, 7, 7, 8, 6]
    );
}
