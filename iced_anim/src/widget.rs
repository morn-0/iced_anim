//! A collection of widgets whose style are implicitly animated.
//!
//! This means the style of these widgets animate changes automatically without any further work
//! from the user. These widgets are generally modified versions of built-in widgets in Iced that
//! have been fitted to include animations by default.
mod animated_state;
pub mod button;

pub use animated_state::AnimatedState;
pub use button::{button, Button};
