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

/// Custom primitive used by [`CWBVH`].
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct PrimitiveCWBVH {
    pub vertex_0: [f32; 3],
    pub original_primitive: u32,
    pub vertex_1: [f32; 3],
    pub padding_0: u32,
    pub vertex_2: [f32; 3],
    pub padding_1: u32,
}

impl Debug for PrimitiveCWBVH {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PrimitiveCWBVH")
            .field("vertex_0", &self.vertex_0)
            .field("vertex_1", &self.vertex_1)
            .field("vertex_2", &self.vertex_2)
            .field("original_primitive", &self.original_primitive)
            .finish()
    }
}

/// CWBVH with node layout [`NodeCWBVH`].
pub struct CWBVH<'a> {
    inner: cxx::UniquePtr<ffi::BVH8_CWBVH>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> CWBVH<'a> {
    pub fn nodes(&self) -> &[NodeCWBVH] {
        // TODO: Create CWBVH node in tinybvh to avoid that.
        let ptr = ffi::CWBVH_nodes(&self.inner) as *const NodeCWBVH;
        let count = ffi::CWBVH_nodes_count(&self.inner);
        unsafe { std::slice::from_raw_parts(ptr, count as usize) }
    }

    /// Encoded primitive data.
    ///
    /// This layout is intersected using a custom primitive array
    /// instead of the original list used during building.
    pub fn primitives(&self) -> &[PrimitiveCWBVH] {
        // TODO: Create struct in tinybvh to avoid that.
        let ptr = ffi::CWBVH_primitives(&self.inner) as *const PrimitiveCWBVH;
        let count = ffi::CWBVH_primitives_count(&self.inner);
        unsafe { std::slice::from_raw_parts(ptr, count as usize) }
    }

    pub fn new_internal() -> Self {
        Self {
            inner: ffi::CWBVH_new(),
            _phantom: PhantomData,
        }
    }
}
super::impl_bvh!(CWBVH, BVH8_CWBVH);
