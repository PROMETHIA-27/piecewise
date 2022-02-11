#![feature(fn_traits, unboxed_closures, generic_associated_types)]

pub mod tuple_traits;
pub mod partial_application;

pub use tuple_traits::*;
pub use partial_application::*;

mod tests;