#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod consume;
mod ext;
mod joined;

pub use crate::{
    consume::Consumed,
    ext::{IntoIteratorByRefExt, IntoIteratorExt},
    joined::Joined,
};

pub mod prelude {
    pub use crate::ext::{IntoIteratorByRefExt as _, IntoIteratorExt as _};
}
