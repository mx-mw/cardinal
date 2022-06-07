pub mod gfx;
pub mod state;
pub mod error;

use error::*;
pub type CardinalResult<T> = Result<T, CardinalError>;

pub enum CardinalError {
    GFX(GFXError),
    State(StateError)
}