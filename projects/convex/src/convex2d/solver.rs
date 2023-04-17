use super::*;

impl<T> FastConvexHull<T> {
    pub fn new(points: &[Point<T>]) -> Self where T: Real {
        // TODO: Optimize using divide and conquer
        graham_scan(points)
    }
}

fn sort_by_min_angle<F: Real>(set: &[Point<F>], min: &Point<F>) -> Vec<Point<F>> {
    let mut points: Vec<(F, F, Point<F>)> = set
        .iter()
        .map(|x| {
            (
                (x.y - min.y).atan2(x.x - min.x),
                // angle
                (x.y - min.y).hypot(x.x - min.x),
                // distance (we want the closest to be first)
                *x,
            )
        })
        .collect();
    points.sort_by(|(a1, d1, _), (a2, d2, _)| (a1, d1).partial_cmp(&(a2, d2)).unwrap_or(Equal));
    points.into_iter().map(|x| x.2).collect()
}

fn z_vector_product<F: Real>(a: &Point<F>, b: &Point<F>, c: &Point<F>) -> F {
    (b.x - a.x) * (c.y - a.y) - (c.x - a.x) * (b.y - a.y)
}

fn graham_scan<F: Real>(set: &[Point<F>]) -> FastConvexHull<F> {
    if set.is_empty() {
        return FastConvexHull {
            bounds: vec![],
            inners: vec![],
        };
    }

    let mut inner = vec![];
    let mut stack: Vec<Point<F>> = vec![];
    let min = set
        .iter()
        .min_by(|a, b| {
            let ord = a.y.partial_cmp(&b.y).unwrap_or(Equal);
            match ord {
                Equal => a.x.partial_cmp(&b.x).unwrap_or(Equal),
                o => o,
            }
        })
        .unwrap();
    let points = sort_by_min_angle(set, min);

    if points.len() <= 3 {
        return FastConvexHull {
            bounds: points,
            inners: inner,
        };
    }

    for point in points {
        while stack.len() > 1
            && z_vector_product(&stack[stack.len() - 2], &stack[stack.len() - 1], &point) <= F::zero()
        {
            // SAFETY: we know that stack.len() > 1
            unsafe {
                inner.push(stack.pop().unwrap_unchecked());
            }
        }
        stack.push(point);
    }

    FastConvexHull {
        bounds: stack,
        inners: inner,
    }
}

impl<T: Real> AddAssign<&[Point<T>]> for FastConvexHull<T> {
    fn add_assign(&mut self, rhs: &[Point<T>]) {
        self.bounds.extend_from_slice(rhs);
        let Self { bounds, inners } = graham_scan(&self.bounds);
        self.bounds = bounds;
        self.inners.extend_from_slice(&inners);
    }
}

impl<T: Real> AddAssign<&FastConvexHull<T>> for FastConvexHull<T> {
    fn add_assign(&mut self, rhs: &FastConvexHull<T>) {
        self.add_assign(rhs.bounds.as_slice());
        self.inners.extend_from_slice(&rhs.inners);
    }
}


#[test]
// from https://codegolf.stackexchange.com/questions/11035/find-the-convex-hull-of-a-set-of-2d-points
fn lots_of_points() {
    let list = vec![
        (4.4, 14.),
        (6.7, 15.25),
        (6.9, 12.8),
        (2.1, 11.1),
        (9.5, 14.9),
        (13.2, 11.9),
        (10.3, 12.3),
        (6.8, 9.5),
        (3.3, 7.7),
        (0.6, 5.1),
        (5.3, 2.4),
        (8.45, 4.7),
        (11.5, 9.6),
        (13.8, 7.3),
        (12.9, 3.1),
        (11., 1.1),
    ];
    let ans = vec![
        (11., 1.1),
        (12.9, 3.1),
        (13.8, 7.3),
        (13.2, 11.9),
        (9.5, 14.9),
        (6.7, 15.25),
        (4.4, 14.),
        (2.1, 11.1),
        (0.6, 5.1),
        (5.3, 2.4),
    ];

    let points = PointSet::from_iter(list).points;
    let ans = PointSet::from_iter(ans).points;
    let ch = graham_scan(&points);
    println!("{:#?}", ch);
    assert_eq!(ch.bounds, ans);
}
