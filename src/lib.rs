//! Rust wrapper for [tinybvh](https://github.com/jbikker/tinybvh).
//!
//! Provides BVH: construction and intersection library
//! - Construction: Wald, BVH4, CWBVH
//! - Intersection

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

pub struct NodeId(pub u32);

impl NodeId {
    pub fn root() -> Self {
        Self(0)
    }

    pub fn new(id: u32) -> Self {
        Self(id)
    }
}
