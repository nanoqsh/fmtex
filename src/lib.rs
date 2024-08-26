#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod consumed;
mod ext;
mod joined;
mod repeated;

pub use crate::{
    consumed::Consumed,
    ext::{IntoIteratorByRefExt, IntoIteratorExt},
    joined::Joined,
    repeated::Repeated,
};

/// Reexported extension traits.
pub mod prelude {
    pub use crate::ext::{DisplayExt as _, IntoIteratorByRefExt as _, IntoIteratorExt as _};
}
