#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod consume;
mod ext;
mod joined;

pub use {
    consume::Consume,
    ext::{FormatRefIterator, IntoIteratorExt},
    joined::Joined,
};
