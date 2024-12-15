//! Rust wrapper for [tinybvh](https://github.com/jbikker/tinybvh).
//!
//! Provides BVH: construction and intersection library
//! - Construction: ['crate::BVH'], ['crate::BVH'], CWBVH
//! - Intersection
//!
//! For more information about each layout: [tinybvh](https://github.com/jbikker/tinybvh).
//!
//! # Examples
//!
//! ```
//! use tinybvh_rs::{BVH, Intersector, Ray};
//!
//! let primitives = vec![
//!     [-2.0, 1.0, -1.0, 0.0],    //
//!     [-1.0, 1.0, -1.0, 0.0],    // Left triangle
//!     [-2.0, 0.0, -1.0, 0.0],    //
//!
//!     [2.0, 1.0, -1.0, 0.0],     //
//!     [2.0, 0.0, -1.0, 0.0],     // Right triangle
//!     [1.0, 0.0, -1.0, 0.0],     //
//! ];
//!
//! let bvh = BVH::new(&primitives);
//!
//! // No intersection, ray pass between the primitives
//! let mut ray = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
//! bvh.intersect(&mut ray);
//! println!("Hit distance: {}", ray.hit.t); // 1e30
//!
//! // Intersects left primitive
//! let mut ray = Ray::new([-1.5, 0.5, 0.0], [0.0, 0.0, -1.0]);
//! bvh.intersect(&mut ray);
//! println!("Hit distance & primtive: {} / {}", ray.hit.t, ray.hit.prim); // 1.0 / 0
//!
//! // Intersects right primitive
//! let mut ray = Ray::new([1.5, 0.45, 0.0], [0.0, 0.0, -1.0]);
//! bvh.intersect(&mut ray);
//! println!("Hit distance & primtive: {} / {}", ray.hit.t, ray.hit.prim); // 1.0 / 1
//! ```
//!
//! # Notes
//!
//! All constructed BVH have a lifetime bound required by tinybvh,
//! which holds to the primitives slice.
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
