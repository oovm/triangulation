use crate::Triangulation;
use shape_core::{Real, Rectangle};
use std::{fmt::Display, path::Path};
use svg::node::{element::SVG, Value};

#[derive(Debug)]
pub struct TriangulationSVG {
    fill_render: bool,
    fill_color: String,
    edge_render: bool,
    edge_color: String,
    edge_width: f32,
    vertex_render: bool,
    vertex_color: String,
}

impl Default for TriangulationSVG {
    fn default() -> Self {
        Self {
            fill_render: true,
            fill_color: "yellow".to_string(),
            edge_render: true,
            edge_color: "black".to_string(),
            edge_width: 0.3,
            vertex_render: true,
            vertex_color: "red".to_string(),
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
    pub fn set_vertex(&mut self, enable: bool, color: &str) {
        self.vertex_render = enable;
        self.vertex_color = color.to_string();
    }
    pub fn with_vertex(mut self, enable: bool, color: &str) -> Self {
        self.set_vertex(enable, color);
        self
    }
}

fn get_area<T>(area: &Rectangle<T>) -> (f32, f32, f32, f32)
where
    T: Real,
{
    let x = area.anchor.x.to_f32().unwrap_or_default();
    let y = area.anchor.y.to_f32().unwrap_or_default();
    let w = area.side.0.to_f32().unwrap_or_default();
    let h = area.side.1.to_f32().unwrap_or_default();
    (x, y, w, h)
}

fn adaptive_point_size<T>(area: &Rectangle<T>) -> f32
where
    T: Real,
{
    let min_side = area.side.0.min(area.side.1);
    min_side.to_f32().unwrap_or_default() / 200.0
}

impl TriangulationSVG {
    pub fn render<T>(&self, set: &Triangulation<T>) -> SVG
    where
        T: Real + Display,
        Value: From<T>,
    {
        let area = set.area;
        let (x, y, w, h) = get_area(&area);
        let point_size = adaptive_point_size(&area);
        let mut out = SVG::new()
            .set("width", w)
            .set("height", h)
            .set("viewBox", format!("{},{},{},{}", x - point_size, y - point_size, w + point_size * 2.0, h + point_size * 2.0));
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
                        .set("stroke-width", point_size * self.edge_width)
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
                        .set("r", point_size)
                        .set("fill", self.vertex_color.clone()),
                );
            }
        }
        out
    }
    pub fn save<P>(path: P, svg: &SVG) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        svg::save(path, svg)
    }
}
