#[cfg(feature = "rand")]
mod with_rand;

#[cfg(feature = "svg")]
mod svg;


#[cfg(feature = "rand")]
pub use self::with_rand::{random64_in_rectangle, random32_in_rectangle, random64_in_ellipse, random32_in_ellipse};

pub use self::svg::{TriangulationSVG};