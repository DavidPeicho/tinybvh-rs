use crate::ffi;
use std::{fmt::Debug, marker::PhantomData};

/// "Traditional" 32-bytes BVH node layout, as proposed by Ingo Wald.
///
/// Node layout used by [`BVH`].
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NodeWald {
    /// AABB min position.
    pub min: [f32; 3],
    /// If the node is a leaf, this is the start index of the primitive.
    /// Otherwise, this is the start index of the first child node.
    pub left_first: u32,
    /// AABB max position.
    pub max: [f32; 3],
    /// If the node is a leaf, number of triangles in the node.
    /// `0` otherwise.
    pub tri_count: u32,
}

impl NodeWald {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

/// BVH with layout [`BVHNode`].
///
/// # Examples
///
/// ```
/// use tinybvh_rs::BVH;
///
/// let triangles = vec![
///     [-1.0, 1.0, 0.0, 0.0],
///     [1.0, 1.0, 0.0, 0.0],
///     [-1.0, 0.0, 0.0, 0.0]
/// ];
/// let bvh = BVH::new(&triangles);
/// ```
pub struct BVH<'a> {
    inner: cxx::UniquePtr<ffi::BVH>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH<'a> {
    pub fn new(primitives: &'a [[f32; 4]]) -> Self {
        BVH {
            inner: ffi::new_bvh(),
            _phantom: Default::default(),
        }
        .build(primitives)
    }

    // Remove unused nodes and reduce the size of the BVH.
    pub fn compact(&mut self) {
        self.inner.pin_mut().Compact();
    }

    /// Number of primitives for a given node.
    pub fn primitive_count(&self, id: u32) -> u32 {
        self.inner.PrimCount(id) as u32
    }

    /// SAH cost for a subtree.
    pub fn sah_cost(&self, id: u32) -> f32 {
        self.inner.SAHCost(id)
    }

    /// BVH nodes.
    ///
    /// Useful to upload to the BVH to the GPU.
    pub fn nodes(&self) -> &[NodeWald] {
        ffi::bvh_nodes(&self.inner)
    }
}
super::impl_bvh!(BVH, BVH);
