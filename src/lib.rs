pub mod error;
pub mod component;
pub mod scene;
pub mod builder;
pub mod util;

pub use util::*;
pub use scene::{Scene, scene};
pub use component::{Component, ComponentRequirements};
pub use builder::start;
use error::*;

pub type CardinalResult<T> = Result<T, CardinalError>;

#[derive(Debug)]
pub enum CardinalError {
    GFX(GFXError),
    State(StateError)
}
