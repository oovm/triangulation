use shape_core::{Itertools, Line, Point, PointSet, Rectangle, Shape2D, Triangle};

#[derive(Debug)]
pub struct Triangulation<T> {
    pub(crate) area: Rectangle<T>,
    pub(crate) points: PointSet<T>,
    pub(crate) vertex_traversal: Vec<usize>,
}

impl<T: Clone> Triangulation<T> {
    pub fn triangles(&self) -> impl Iterator<Item = Triangle<T>> + '_ {
        self.triangle_vertexes().map(|[a, b, c]| Triangle::new(a, b, c))
    }
    pub fn triangle_vertexes(&self) -> impl Iterator<Item = [Point<T>; 3]> + '_ {
        self.vertex_traversal
            .iter()
            .tuple_windows::<(usize, usize, usize)>() // 3 points per triangle
            .map(move |(a, b, c)| [self.points[a].clone(), self.points[b].clone(), self.points[c].clone()])
    }
    pub fn get_indexes(&self) -> &[usize] {
        &self.vertex_traversal
    }
    pub fn triangle_count(&self) -> usize {
        self.vertex_traversal.len() / 3
    }
    pub fn get_points(&self) -> &[Point<T>] {
        &self.points
    }
    pub fn get_area(&self) -> Rectangle<T> {
        self.area.clone()
    }
}

fn triangulate_2d_f64(points: PointSet<f64>) -> Triangulation<f64> {
    let cast = points.points.iter().map(|p| delaunator::Point { x: p.x, y: p.y }).collect_vec();
    let result = delaunator::triangulate(&cast);

    Triangulation { area: points.boundary(), points, vertex_traversal: result.triangles }
}
