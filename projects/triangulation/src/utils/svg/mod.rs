use crate::Triangulation;
use shape_core::{Real, Rectangle};
use shape_svg::{ToSVG, SVG};
use std::{fmt::Display, path::Path};

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
    let x = area.x.to_f32().unwrap_or_default();
    let y = area.y.to_f32().unwrap_or_default();
    let w = area.w.to_f32().unwrap_or_default();
    let h = area.h.to_f32().unwrap_or_default();
    (x, y, w, h)
}

fn adaptive_point_size<T>(area: &Rectangle<T>) -> f32
where
    T: Real,
{
    let min_side = area.w.min(area.h);
    min_side.to_f32().unwrap_or_default() / 200.0
}

impl TriangulationSVG {
    pub fn render<T>(&self, set: &Triangulation<T>) -> SVG
    where
        T: Real + Display,
    {
        let area = set.area;
        let (x, y, w, h) = get_area(&area);
        let point_size = adaptive_point_size(&area);
        let mut out = SVG::new()
            .set("width", w.to_string())
            .set("height", h.to_string())
            .set("viewBox", format!("{},{},{},{}", x - point_size, y - point_size, w + point_size * 2.0, h + point_size * 2.0));
        if self.fill_render {
            for triangle in set.triangles() {
                out = out.add(triangle.to_svg().set("fill", self.fill_color.clone()).set("stroke", "none"));
            }
        }
        if self.edge_render {
            for line in set.edges() {
                out = out.add(
                    line.to_svg()
                        .set("stroke", self.edge_color.clone())
                        .set("stroke-width", self.edge_width)
                        .set("fill", "none"),
                );
            }
        }
        if self.vertex_render {
            for point in set.vertices() {
                out = out.add(point.to_svg().set("fill", self.vertex_color.clone()).set("stroke", "none").set("r", point_size));
            }
        }
        out
    }
    pub fn save<P>(path: P, svg: &SVG) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        shape_svg::save(path, svg)
    }
}
