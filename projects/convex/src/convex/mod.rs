use shape_core::{Point, PointSet, Real};
use std::cmp::Ordering::Equal;
use std::fmt::Debug;
use std::ops::AddAssign;

#[derive(Debug)]
pub struct ConvexHull<T> {
    bounds: Vec<Point<T>>,
    inners: Vec<Point<T>>,
}

mod solver;

impl<T> ConvexHull<T> {
    /// Clear inner points in the convex hull
    pub fn clear(&mut self) {
        self.inners.clear();
    }
}

impl<T: Real> AddAssign<&[Point<T>]> for ConvexHull<T> {
    fn add_assign(&mut self, rhs: &[Point<T>]) {
        self.bounds.extend_from_slice(rhs);
        let Self { bounds, inners } = graham_scan(&self.bounds);
        self.bounds = bounds;
        self.inners.extend_from_slice(&inners);
    }
}

impl<T: Real> AddAssign<&ConvexHull<T>> for ConvexHull<T> {
    fn add_assign(&mut self, rhs: &ConvexHull<T>) {
        self.add_assign(rhs.bounds.as_slice());
        self.inners.extend_from_slice(&rhs.inners);
    }
}

