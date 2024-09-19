#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod display;
mod iter;

pub use crate::{
    display::{DisplayExt, Repeated},
    iter::{Consumed, IntoIteratorByRefExt, IntoIteratorExt, Joined},
};

/// Reexported extension traits.
pub mod prelude {
    pub use crate::{
        display::DisplayExt as _,
        iter::{IntoIteratorByRefExt as _, IntoIteratorExt as _},
    };
}
