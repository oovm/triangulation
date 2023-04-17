mod dim2;
mod dim3;
#[cfg(test)]
mod tests;

pub struct ConvexHull {

}

/// The shape-mesh hull is merge-able
pub trait ConvexHullMerge<T>
where
    Self: Sized,
{
    /// Helper function to combine two shape-mesh hulls into one
    ///
    /// This is used when using the divide and conquer algorithm
    fn merge_convex_hulls(&mut self, rhs: Self, tolerance: Option<T>) -> Self {
        match self.is_intersect(&rhs, &tolerance) {
            true => self.merge_intersect(rhs, tolerance),
            false => self.merge_standalone(rhs, tolerance),
        }
    }
    /// Helper function to check if two shape-mesh hulls is intersect.
    ///
    /// This is used when using the divide and conquer algorithm
    fn is_intersect(&self, other: &Self, tolerance: &Option<T>) -> bool;

    fn merge_intersect(&mut self, rhs: Self, tolerance: Option<T>) -> Self;

    fn merge_standalone(&mut self, rhs: Self, tolerance: Option<T>) -> Self;
}

///
pub trait ConvexSet<T> {
    ///
    type Output;
    ///
    fn get_convex_set(&self, tolerance: Option<T>) -> Option<Self::Output>;
}
