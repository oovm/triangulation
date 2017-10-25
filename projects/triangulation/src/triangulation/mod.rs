use shape_core::{Line, Point, Rectangle, Triangle};

mod with_svg;
mod with_random;

#[derive(Debug)]
pub struct Triangulation<T> {
    pub(crate) area: Rectangle<T>,
    pub(crate) point: Vec<Point<T>>,
    pub(crate) vertex_traversal: Vec<usize>,
    pub(crate) edge_traversal: Vec<usize>,
}

impl<T: Clone> Triangulation<T> {
    pub fn triangles(&self) -> impl Iterator<Item = Triangle<T>> + '_ {
        self.triangle_vertexes().map(|[a, b, c]| Triangle::new(a, b, c))
    }
    pub fn triangle_vertexes(&self) -> impl Iterator<Item = [Point<T>; 3]> + '_ {
        self.vertex_traversal
            .chunks_exact(3) // 3 points per triangle
            .map(move |chunk| [self.point[chunk[0]].clone(), self.point[chunk[1]].clone(), self.point[chunk[2]].clone()])
    }
    pub fn edges(&self) -> impl Iterator<Item = Line<T>> + '_ {
        vec![].into_iter()
    }
    pub fn get_indexes(&self) -> &[usize] {
        &self.vertex_traversal
    }
    pub fn triangle_count(&self) -> usize {
        self.vertex_traversal.len() / 3
    }
    pub fn get_points(&self) -> &[Point<T>] {
        &self.point
    }
    pub fn get_area(&self) -> Rectangle<T> {
        self.area.clone()
    }
}
