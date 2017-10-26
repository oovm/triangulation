use std::fmt::Display;
use std::path::Path;
use shape_core::Real;
use svg::node::element::SVG;
use svg::node::Value;
use crate::Triangulation;

#[derive(Debug)]
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
            fill_color: "yellow".to_string(),
            edge_render: true,
            edge_color: "black".to_string(),
            edge_width: 1.0,
            vertex_render: true,
            vertex_color: "red".to_string(),
            vertex_size: 2.0,
        }
    }
}

impl TriangulationSVG {
    pub fn set_fill(&mut self, enable: bool, color: &str) {
        self.fill_render = enable;
        self.fill_color = color.to_string();
    }
    pub fn with_fill(mut self, enable: bool, color: &str) -> Self {
        self.set_fill(enable, color);
        self
    }
    pub fn set_edge(&mut self, enable: bool, color: &str, width: f32) {
        self.edge_render = enable;
        self.edge_color = color.to_string();
        self.edge_width = width;
    }
    pub fn with_edge(mut self, enable: bool, color: &str, width: f32) -> Self {
        self.set_edge(enable, color, width);
        self
    }
    pub fn set_vertex(&mut self, enable: bool, color: &str, size: f32) {
        self.vertex_render = enable;
        self.vertex_color = color.to_string();
        self.vertex_size = size;
    }
    pub fn with_vertex(mut self, enable: bool, color: &str, size: f32) -> Self {
        self.set_vertex(enable, color, size);
        self
    }
}


impl TriangulationSVG {
    pub fn render<T>(&self, set: &Triangulation<T>) -> SVG where T: Real + Display, Value: From<T> {
        let area = set.get_area();
        let mut out = SVG::new()
            .set("width", area.side.0)
            .set("height", area.side.1)
            .set("viewBox", format!("{} {} {} {}", area.anchor.x, area.anchor.y, area.side.0, area.side.1));
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
            for [a, b, c] in set.triangle_vertexes() {
                out = out.add(
                    svg::node::element::Path::new()
                        .set("fill", "none")
                        .set("stroke", self.edge_color.clone())
                        .set("stroke-width", self.edge_width)
                        .set("d", format!("M {},{} L {},{} L {},{} Z", a.x, a.y, b.x, b.y, c.x, c.y)),
                );
            }
        }
        if self.vertex_render {
            for a in set.get_points() {
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
    pub fn save<P>(path: P, svg: &SVG) -> std::io::Result<()>
        where P: AsRef<Path> {
        svg::save(path, svg)
    }
}
