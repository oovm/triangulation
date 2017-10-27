#[cfg(feature = "rand")]
mod with_rand;

#[cfg(feature = "svg")]
mod svg;

#[cfg(feature = "rand")]
pub use self::with_rand::{random32_in_ellipse, random32_in_rectangle, random64_in_ellipse, random64_in_rectangle};

#[cfg(feature = "svg")]
pub use self::svg::TriangulationSVG;
