pub mod error;
pub mod generator;
pub mod templates;
pub mod net_generator;
pub mod next_generator;

pub use error::{Error, Result};
pub use generator::Generator;

#[cfg(test)]
mod generator_test;

#[cfg(test)]
mod net_generator_test;

#[cfg(test)]
mod next_generator_test;