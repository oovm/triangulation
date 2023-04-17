#![deny(rustdoc::missing_crate_level_docs)]
#![warn(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str!("../readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

// pub use crate::traits::*;

// mod extensions;
// mod traits;
// mod utils;
mod convex2d;

pub use convex2d::{FastConvexHull, ConvexHull};