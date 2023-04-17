use shape_core::{Point, PointSet, Polygon, Real};
use std::cmp::Ordering::Equal;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::ops::AddAssign;

/// A convex hull which can't delete points dynamically
///
/// # Notice
///
/// It can merge with another convex hull, but it can't delete points inside
#[derive(Debug)]
pub struct FastConvexHull<T> {
    bounds: Vec<Point<T>>,
    inners: Vec<Point<T>>,
}


/// A convex hull which delete points dynamically, internally with balance tree
#[derive(Debug)]
pub struct ConvexHull<T> {
    bounds: BTreeSet<Point<T>>,
    inners: BTreeSet<Point<T>>,
}

mod solver;

impl<T> FastConvexHull<T> {
    /// Clear inner points in the convex2d hull
    pub fn clear(&mut self) {
        self.inners.clear();
    }
    pub fn bound_points(&self) -> impl Iterator<Item=Point<&T>> {
        self.bounds.iter().map(|v| v.ref_inner())
    }
    pub fn inner_points(&self) -> impl Iterator<Item=Point<&T>> {
        self.inners.iter().map(|v| v.ref_inner())
    }
    pub fn as_polygon(&self) -> Polygon<T> where T: Clone {
        Polygon {
            points_set: PointSet { points: self.bounds.clone() },
        }
    }
}
