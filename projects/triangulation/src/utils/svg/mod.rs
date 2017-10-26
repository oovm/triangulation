use std::path::Path;
use svg::node::element::SVG;
use crate::Triangulation;

pub struct TriangulationSVG {
    fill_render: bool,
    fill_color: String,
    edge_render: bool,
    edge_color: String,
    edge_width: f32,
    vertex_render: bool,
    vertex_color: String,
    vertex_size: f32,
}

impl Default for TriangulationSVG {
    fn default() -> Self {
        Self {
            fill_render: true,
            fill_color: "black".to_string(),
            edge_render: true,
            edge_color: "black".to_string(),
            edge_width: 1.0,
            vertex_render: true,
            vertex_color: "black".to_string(),
            vertex_size: 2.0,
        }
    }
}


impl TriangulationSVG {
    pub fn render<T>(&self, set: &Triangulation<T>) -> SVG {
        let mut out = SVG::new();
        if self.fill_render {
            for [a, b, c] in set.triangle_vertexes() {
                out = out.add(
                    svg::node::element::Path::new()
                        .set("fill", self.fill_color.clone())
                        .set("stroke", "none")
                        .set("d", format!("M {},{} L {},{} L {},{} Z", a.x, a.y, b.x, b.y, c.x, c.y)),
                );
            }
        }
        if self.edge_render {
            for [a, b] in set.edge_vertexes() {
                out = out.add(
                    svg::node::element::Path::new()
                        .set("fill", "none")
                        .set("stroke", self.edge_color.clone())
                        .set("stroke-width", self.edge_width)
                        .set("d", format!("M {},{} L {},{}", a.x, a.y, b.x, b.y)),
                );
            }
        }
        if self.vertex_render {
            for a in set.vertexes() {
                out = out.add(
                    svg::node::element::Circle::new()
                        .set("cx", a.x)
                        .set("cy", a.y)
                        .set("r", self.vertex_size)
                        .set("fill", self.vertex_color.clone()),
                );
            }
        }
        out
    }
    pub fn save<P>(&self, path: P, svg: &SVG) where P: AsRef<Path> {
        svg::save(path, svg).unwrap();
    }
}
