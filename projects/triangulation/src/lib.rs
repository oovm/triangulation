#![deny(rustdoc::missing_crate_level_docs)]
// #![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc = include_str ! ("../Readme.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]
#![doc(html_favicon_url = "https://raw.githubusercontent.com/oovm/shape-rs/dev/projects/images/Trapezohedron.svg")]

pub use crate::triangulation::*;

mod delaunay2f32;
mod delaunay2f64;
mod triangulation;

pub use crate::{delaunay2f32::triangulate_2d_f32, delaunay2f64::triangulate_2d_f64, triangulation::Triangulation};
