use shape_core::Point;
use svg::node::element::SVG;
use shape_triangulation::triangulate_2d_f32;
use shape_triangulation::utils::random64_in_circle;

pub struct SvgRendererConfig {
    fill_render: bool,
    fill_color: String,
    edge_render: bool,
    edge_color: String,
    edge_width: f32,
    vertex_render: bool,
    vertex_color: String,
    vertex_size: f32,
}

#[test]
fn test() {
    let points = randf();
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
