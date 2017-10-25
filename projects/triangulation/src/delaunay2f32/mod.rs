//! Raw impl from https://github.com/mourner/delaunator-rs/blob/master/src/lib.rs

use crate::Triangulation;
use core::cmp::Ordering;
use shape_core::{Circle, Point, Rectangle};

/// Represents the area outside of the delaunay2f32.
/// Halfedges on the convex hull (which don't have an adjacent halfedge)
/// will have this value.
pub const EMPTY: usize = usize::MAX;

/// Returns a **negative** value if ```self```, ```q``` and ```r``` occur in counterclockwise order (```r``` is to the left of the directed line ```self``` --> ```q```)
/// Returns a **positive** value if they occur in clockwise order(```r``` is to the right of the directed line ```self``` --> ```q```)
/// Returns zero is they are collinear
fn orient(a: &Point<f32>, b: &Point<f32>, c: &Point<f32>) -> f32 {
    -(b.y - a.y) * (c.x - b.x) + (b.x - a.x) * (c.y - b.y)
}

/// Near-duplicate points (where both `x` and `y` only differ within this value)
/// will not be included in the delaunay2f32 for robustness.
fn nearly_equals(a: &Point<f32>, b: &Point<f32>) -> bool {
    (a.x - b.x).abs() <= f32::EPSILON * 2.0 && (a.y - b.y).abs() <= f32::EPSILON * 2.0
}

/// Next half edge in a triangle.
pub fn next_half_edge(i: usize) -> usize {
    if i % 3 == 2 { i - 2 } else { i + 1 }
}

/// Previous half edge in a triangle.
pub fn prev_half_edge(i: usize) -> usize {
    if i % 3 == 0 { i + 2 } else { i - 1 }
}

/// Result of the Delaunay delaunay2f32.
#[derive(Debug, Clone)]
struct TriangulationState {
    /// A vector of point indices where each triple represents a Delaunay triangle.
    /// All triangles are directed counter-clockwise.
    pub triangles: Vec<usize>,
    /// A vector of adjacent halfedge indices that allows traversing the delaunay2f32 graph.
    ///
    /// `i`-th half-edge in the array corresponds to vertex `triangles[i]`
    /// the half-edge is coming from. `halfedges[i]` is the index of a twin half-edge
    /// in an adjacent triangle (or `EMPTY` for outer half-edges on the convex hull).
    pub half_edges: Vec<usize>,
    /// A vector of indices that reference points on the convex hull of the delaunay2f32,
    /// counter-clockwise.
    pub hull: Vec<usize>,
}

impl TriangulationState {
    fn new(n: usize) -> Self {
        let max_triangles = if n > 2 { 2 * n - 5 } else { 0 };
        Self {
            triangles: Vec::with_capacity(max_triangles * 3),
            half_edges: Vec::with_capacity(max_triangles * 3),
            hull: Vec::new(),
        }
    }

    fn add_triangle(&mut self, i0: usize, i1: usize, i2: usize, a: usize, b: usize, c: usize) -> usize {
        let t = self.triangles.len();

        self.triangles.push(i0);
        self.triangles.push(i1);
        self.triangles.push(i2);

        self.half_edges.push(a);
        self.half_edges.push(b);
        self.half_edges.push(c);

        if a != EMPTY {
            self.half_edges[a] = t;
        }
        if b != EMPTY {
            self.half_edges[b] = t + 1;
        }
        if c != EMPTY {
            self.half_edges[c] = t + 2;
        }

        t
    }

    fn legalize(&mut self, a: usize, points: &[Point<f32>], hull: &mut Hull) -> usize {
        let b = self.half_edges[a];

        // if the pair of triangles doesn't satisfy the Delaunay condition
        // (p1 is inside the circumcircle of [p0, pl, pr]), flip them,
        // then do the same check/flip recursively for the new pair of triangles
        //
        //           pl                    pl
        //          /||\                  /  \
        //       al/ || \bl            al/    \a
        //        /  ||  \              /      \
        //       /  a||b  \    flip    /___ar___\
        //     p0\   ||   /p1   =>   p0\---bl---/p1
        //        \  ||  /              \      /
        //       ar\ || /br             b\    /br
        //          \||/                  \  /
        //           pr                    pr
        //
        let ar = prev_half_edge(a);

        if b == EMPTY {
            return ar;
        }

        let al = next_half_edge(a);
        let bl = prev_half_edge(b);

        let p0 = self.triangles[ar];
        let pr = self.triangles[a];
        let pl = self.triangles[al];
        let p1 = self.triangles[bl];
        if Circle::from_3_points(&points[p0], &points[pr], &points[pl]).contains(&points[p1]) {
            self.triangles[a] = p1;
            self.triangles[b] = p0;

            let hbl = self.half_edges[bl];
            let har = self.half_edges[ar];

            // edge swapped on the other side of the hull (rare); fix the halfedge reference
            if hbl == EMPTY {
                let mut e = hull.start;
                loop {
                    if hull.tri[e] == bl {
                        hull.tri[e] = a;
                        break;
                    }
                    e = hull.prev[e];
                    if e == hull.start {
                        break;
                    }
                }
            }

            self.half_edges[a] = hbl;
            self.half_edges[b] = har;
            self.half_edges[ar] = bl;

            if hbl != EMPTY {
                self.half_edges[hbl] = a;
            }
            if har != EMPTY {
                self.half_edges[har] = b;
            }
            if bl != EMPTY {
                self.half_edges[bl] = ar;
            }

            let br = next_half_edge(b);

            self.legalize(a, points, hull);
            return self.legalize(br, points, hull);
        }
        ar
    }
}

// data structure for tracking the edges of the advancing convex hull
struct Hull {
    prev: Vec<usize>,
    next: Vec<usize>,
    tri: Vec<usize>,
    hash: Vec<usize>,
    start: usize,
    center: Point<f32>,
}

impl Hull {
    fn new(n: usize, center: Point<f32>, i0: usize, i1: usize, i2: usize, points: &[Point<f32>]) -> Self {
        let hash_len = (n as f32).sqrt() as usize;

        let mut hull = Self {
            prev: vec![0; n],            // edge to prev edge
            next: vec![0; n],            // edge to next edge
            tri: vec![0; n],             // edge to adjacent halfedge
            hash: vec![EMPTY; hash_len], // angular edge hash
            start: i0,
            center,
        };

        hull.next[i0] = i1;
        hull.prev[i2] = i1;
        hull.next[i1] = i2;
        hull.prev[i0] = i2;
        hull.next[i2] = i0;
        hull.prev[i1] = i0;

        hull.tri[i0] = 0;
        hull.tri[i1] = 1;
        hull.tri[i2] = 2;

        hull.hash_edge(&points[i0], i0);
        hull.hash_edge(&points[i1], i1);
        hull.hash_edge(&points[i2], i2);

        hull
    }

    fn hash_key(&self, p: &Point<f32>) -> usize {
        let dx = p.x - self.center.x;
        let dy = p.y - self.center.y;

        let p = dx / ((dx).abs() + (dy).abs());
        let a = (if dy > 0.0 { 3.0 - p } else { 1.0 + p }) / 4.0; // [0..1]

        let len = self.hash.len();
        (((len as f32) * a).floor() as usize) % len
    }

    fn hash_edge(&mut self, p: &Point<f32>, i: usize) {
        let key = self.hash_key(p);
        self.hash[key] = i;
    }

    fn find_visible_edge(&self, p: &Point<f32>, points: &[Point<f32>]) -> (usize, bool) {
        let mut start: usize = 0;
        let key = self.hash_key(p);
        let len = self.hash.len();
        for j in 0..len {
            start = self.hash[(key + j) % len];
            if start != EMPTY && self.next[start] != EMPTY {
                break;
            }
        }
        start = self.prev[start];
        let mut e = start;

        while orient(p, &points[e], &points[self.next[e]]) <= 0. {
            e = self.next[e];
            if e == start {
                return (EMPTY, false);
            }
        }
        (e, e == start)
    }
}

fn calc_bbox_center(points: &[Point<f32>]) -> Point<f32> {
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    for p in points.iter() {
        min_x = min_x.min(p.x);
        min_y = min_y.min(p.y);
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }
    Point { x: (min_x + max_x) / 2.0, y: (min_y + max_y) / 2.0 }
}

fn find_closest_point(points: &[Point<f32>], p0: &Point<f32>) -> Option<usize> {
    let mut min_dist = f32::INFINITY;
    let mut k: usize = 0;
    for (i, p) in points.iter().enumerate() {
        let d = p0.euclidean2(p);
        if d > 0.0 && d < min_dist {
            k = i;
            min_dist = d;
        }
    }
    if min_dist == f32::INFINITY { None } else { Some(k) }
}

fn find_seed_triangle(points: &[Point<f32>]) -> Option<(usize, usize, usize)> {
    // pick a seed point close to the center
    let bbox_center = calc_bbox_center(points);
    let i0 = find_closest_point(points, &bbox_center)?;
    let p0 = &points[i0];

    // find the point closest to the seed
    let i1 = find_closest_point(points, p0)?;
    let p1 = &points[i1];

    // find the third point which forms the smallest circumcircle with the first two
    let mut min_radius = f32::INFINITY;
    let mut i2: usize = 0;
    for (i, p) in points.iter().enumerate() {
        if i == i0 || i == i1 {
            continue;
        }
        let r = Circle::from_3_points(p0, p1, p).radius;
        if r < min_radius {
            i2 = i;
            min_radius = r;
        }
    }

    if min_radius == f32::INFINITY {
        None
    }
    else {
        // swap the order of the seed points for counter-clockwise orientation
        Some(if orient(p0, p1, &points[i2]) > 0. { (i0, i2, i1) } else { (i0, i1, i2) })
    }
}

fn sortf(f: &mut [(usize, f32)]) {
    f.sort_unstable_by(|&(_, da), &(_, db)| da.partial_cmp(&db).unwrap_or(Ordering::Equal));
}

/// Order collinear points by dx (or dy if all x are identical) and return the list as a hull
fn handle_collinear_points(points: &[Point<f32>]) -> Triangulation<f32> {
    let Point { x, y } = points.first().cloned().unwrap_or_default();

    let mut dist: Vec<_> = points
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let mut d = p.x - x;
            if d == 0.0 {
                d = p.y - y;
            }
            (i, d)
        })
        .collect();
    sortf(&mut dist);

    let mut triangulation = TriangulationState::new(0);
    let mut d0 = f32::NEG_INFINITY;
    for (i, distance) in dist {
        if distance > d0 {
            triangulation.hull.push(i);
            d0 = distance;
        }
    }

    Triangulation {
        area: Rectangle { anchor: Point::default(), side: (Default::default(), Default::default()) },
        point: vec![],
        vertex_traversal: triangulation.triangles,
        edge_traversal: vec![],
    }
}

/// Triangulate a set of 2D points.
/// Returns the delaunay2f32 for the input points.
/// For the degenerated case when all points are collinear, returns an empty delaunay2f32 where all points are in the hull.
pub fn triangulate_2d_f32(points: &[Point<f32>]) -> Triangulation<f32> {
    let seed_triangle = find_seed_triangle(points);
    if seed_triangle.is_none() {
        return handle_collinear_points(points);
    }

    let n = points.len();
    let (i0, i1, i2) = seed_triangle.expect("At this stage, points are guaranteed to yeild a seed triangle");
    let center = Circle::from_3_points(&points[i0], &points[i1], &points[i2]).center;

    let mut triangulation = TriangulationState::new(n);
    triangulation.add_triangle(i0, i1, i2, EMPTY, EMPTY, EMPTY);

    // sort the points by distance from the seed triangle circumcenter
    let mut dists: Vec<_> = points.iter().enumerate().map(|(i, point)| (i, center.euclidean2(point))).collect();

    sortf(&mut dists);

    let mut hull = Hull::new(n, center, i0, i1, i2, points);

    for (k, &(i, _)) in dists.iter().enumerate() {
        let p = &points[i];

        // skip near-duplicates
        if k > 0 && nearly_equals(p, &points[dists[k - 1].0]) {
            continue;
        }
        // skip seed triangle points
        if i == i0 || i == i1 || i == i2 {
            continue;
        }

        // find a visible edge on the convex hull using edge hash
        let (mut e, walk_back) = hull.find_visible_edge(p, points);
        if e == EMPTY {
            continue; // likely a near-duplicate point; skip it
        }

        // add the first triangle from the point
        let t = triangulation.add_triangle(e, i, hull.next[e], EMPTY, EMPTY, hull.tri[e]);

        // recursively flip triangles from the point until they satisfy the Delaunay condition
        hull.tri[i] = triangulation.legalize(t + 2, points, &mut hull);
        hull.tri[e] = t; // keep track of boundary triangles on the hull

        // walk forward through the hull, adding more triangles and flipping recursively
        let mut n = hull.next[e];
        loop {
            let q = hull.next[n];
            if orient(p, &points[n], &points[q]) <= 0. {
                break;
            }
            let t = triangulation.add_triangle(n, i, q, hull.tri[i], EMPTY, hull.tri[n]);
            hull.tri[i] = triangulation.legalize(t + 2, points, &mut hull);
            hull.next[n] = EMPTY; // mark as removed
            n = q;
        }

        // walk backward from the other side, adding more triangles and flipping
        if walk_back {
            loop {
                let q = hull.prev[e];
                if orient(p, &points[q], &points[e]) <= 0. {
                    break;
                }
                let t = triangulation.add_triangle(q, i, e, EMPTY, hull.tri[e], hull.tri[q]);
                triangulation.legalize(t + 2, points, &mut hull);
                hull.tri[q] = t;
                hull.next[e] = EMPTY; // mark as removed
                e = q;
            }
        }

        // update the hull indices
        hull.prev[i] = e;
        hull.next[i] = n;
        hull.prev[n] = i;
        hull.next[e] = i;
        hull.start = e;

        // save the two new edges in the hash table
        hull.hash_edge(p, i);
        hull.hash_edge(&points[e], e);
    }

    // expose hull as a vector of point indices
    let mut e = hull.start;
    loop {
        triangulation.hull.push(e);
        e = hull.next[e];
        if e == hull.start {
            break;
        }
    }

    triangulation.triangles.shrink_to_fit();
    triangulation.half_edges.shrink_to_fit();

    Triangulation {
        area: Rectangle::bound_box(points),
        point: points.to_vec(),
        vertex_traversal: triangulation.triangles,
        edge_traversal: vec![],
    }
}
