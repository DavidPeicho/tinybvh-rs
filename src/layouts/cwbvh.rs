use crate::ffi;
use std::{fmt::Debug, marker::PhantomData};

/// Format specified in:
/// "Efficient Incoherent Ray Traversal on GPUs Through Compressed Wide BVHs", Ylitie et al. 2017.
///
/// Node layout used by [`CWBVH`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NodeCWBVH {
    /// AABB min.
    pub min: [f32; 3],
    /// Exponent used for child AABB decompression.
    pub exyz: [u8; 3],
    /// `1` for node, `0` for leaf.
    pub imask: u8,
    /// First child index.
    pub child_base_idx: u32,
    // First primitive index.
    pub primitive_base_idx: u32,
    /// Child [0..7] metadata.
    pub child_meta: [u8; 8],
    // AABB minimum x-axis compressed bound, one entry per child.
    pub qlo_x: [u8; 8],
    // AABB minimum y-axis compressed bound, one entry per child.
    pub qlo_y: [u8; 8],
    // AABB minimum z-axis compressed bound, one entry per child.
    pub qlo_z: [u8; 8],
    // AABB maximum x-axis compressed bound, one entry per child.
    pub qhi_x: [u8; 8],
    // AABB maximum y-axis compressed bound, one entry per child.
    pub qhi_y: [u8; 8],
    // AABB maximum z-axis compressed bound, one entry per child.
    pub qhi_z: [u8; 8],
}

impl NodeCWBVH {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.imask == 0
    }
}

/// CWBVH with node layout [`NodeCWBVH`].
pub struct CWBVH<'a> {
    inner: cxx::UniquePtr<ffi::BVH8_CWBVH>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> CWBVH<'a> {
    pub fn new(primitives: &'a [[f32; 4]]) -> Self {
        Self {
            inner: ffi::cwbvh_new(),
            _phantom: PhantomData,
        }
        .build(primitives)
    }

    pub fn nodes(&self) -> &[NodeCWBVH] {
        // TODO: Create CWBVH node in tinybvh to avoid that.
        let ptr = ffi::cwbvh_nodes(&self.inner) as *const NodeCWBVH;
        let count = ffi::cwbvh_nodes_count(&self.inner);
        unsafe { std::slice::from_raw_parts(ptr, count as usize) }
    }
}
super::impl_bvh!(CWBVH, BVH8_CWBVH);
