#[cfg(feature = "rand")]
mod with_rand;

pub use self::with_rand::{random64_in_rectangle, random32_in_rectangle, random64_in_circle, random32_in_circle};