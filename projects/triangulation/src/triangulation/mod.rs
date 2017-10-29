use shape_core::{Itertools, Line, Point, PointSet, Rectangle, Shape2D, Triangle};
use std::{collections::BTreeSet, iter::from_generator};

#[derive(Debug)]
pub struct Triangulation<T> {
    pub(crate) area: Rectangle<T>,
    pub(crate) points: PointSet<T>,
    pub(crate) vertex_traversal: Vec<usize>,
}

impl<T> Triangulation<T> {
    pub fn boundary(&self) -> Rectangle<&T> {
        self.area.ref_inner()
    }
    pub fn vertices(&self) -> impl Iterator<Item = Point<&T>> + '_ {
        self.points.points.iter().map(|p| p.ref_inner())
    }
    pub fn edges(&self) -> impl Iterator<Item = Line<&T>> + '_ {
        let mut unique = BTreeSet::new();
        from_generator(move || {
            for (a, b, c) in self.vertex_traversal.iter().tuple_windows::<(_, _, _)>() {
                if let Some(line) = self.unique_edges(&mut unique, *a, *b) {
                    yield line;
                }
                if let Some(line) = self.unique_edges(&mut unique, *b, *c) {
                    yield line;
                }
                if let Some(line) = self.unique_edges(&mut unique, *c, *a) {
                    yield line;
                }
            }
        })
    }
    fn unique_edges(&self, set: &mut BTreeSet<(usize, usize)>, a: usize, b: usize) -> Option<Line<&T>> {
        let (a, b) = if a < b { (a, b) } else { (b, a) };
        match set.get(&(a, b)) {
            Some(_) => None,
            None => {
                set.insert((a, b));
                unsafe { Some(Line { s: self.get_indexed(a), e: self.get_indexed(b) }) }
            }
        }
    }

    pub fn triangles(&self) -> impl Iterator<Item = Triangle<&T>> + '_ {
        self.vertex_traversal
            .iter()
            .tuple_windows::<(_, _, _)>() // 3 points per triangle
            .map(|(a, b, c)| unsafe { Triangle { a: self.get_indexed(*a), b: self.get_indexed(*b), c: self.get_indexed(*c) } })
    }
    unsafe fn get_indexed(&self, index: usize) -> Point<&T> {
        self.points.points.get_unchecked(index).ref_inner()
    }
    pub fn count_triangles(&self) -> usize {
        self.vertex_traversal.len() / 3
    }
}

pub fn triangulate_2d_f64(points: PointSet<f64>) -> Triangulation<f64> {
    let cast = points.points.iter().map(|p| delaunator::Point { x: p.x, y: p.y }).collect_vec();
    let result = delaunator::triangulate(&cast);

    Triangulation { area: points.boundary(), points, vertex_traversal: result.triangles }
}
