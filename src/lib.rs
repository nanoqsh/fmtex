#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod consumed;
mod ext;
mod joined;

pub use crate::{
    consumed::Consumed,
    ext::{IntoIteratorByRefExt, IntoIteratorExt},
    joined::Joined,
};

/// Reexported extension traits.
pub mod prelude {
    pub use crate::ext::{IntoIteratorByRefExt as _, IntoIteratorExt as _};
}
