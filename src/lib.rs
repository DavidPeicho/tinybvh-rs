#![cfg_attr(not(doctest), doc = include_str!("../README.md"))]

//! # Notes
//!
//! All constructed BVH have a lifetime bound required by tinybvh, which holds to the primitives slice.

mod cxx_ffi;
mod layouts;
mod ray;
mod traversal;

pub(crate) use cxx_ffi::ffi;
pub use layouts::*;
pub use ray::*;
pub use traversal::*;

/// Infinite value used for intersection.
///
/// **NOTE**: This is not the same as `f32::MAX`.
pub const INFINITE: f32 = 1e30; // Actual valid ieee range: 3.40282347E+38

/// Alias for a strided slice of positions.
///
/// Positions do not need to be strided, but the API accepts a strided
/// slice to support both use cases.
///
/// tinybvh-rs internally requires positions to be vectors of size **4**
/// and not **3**. This is a requirement of the underlying tinybvh library.
pub type Positions<'a> = pas::Slice<'a, [f32; 4]>;
