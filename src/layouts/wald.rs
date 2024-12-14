use std::{fmt::Debug, marker::PhantomData, slice::from_raw_parts};

use crate::{ffi, NodeId};

/// "Traditional" 32-bytes BVH node layout, as proposed by Ingo Wald.
///
/// Node layout used by [`BVH`].
///
/// For more information: [tinybvh](https://github.com/jbikker/tinybvh).
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BVHNode {
    /// AABB min value.
    pub min: [f32; 3],
    /// If the node is a leaf, this is the start index of the primitive.
    /// Otherwise, this is the start index of the first child node.
    pub left_first: u32,
    /// AABB max value.
    pub max: [f32; 3],
    /// If the node is a leaf, number of triangles in the node.
    /// `0` otherwise.
    pub tri_count: u32,
}

impl BVHNode {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

impl Debug for BVHNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BVHNode")
            .field("min", &self.min)
            .field("left_first", &self.left_first)
            .field("max", &self.max)
            .field("tri_count", &self.tri_count)
            .finish()
    }
}

/// BVH with layout [`BVHNode`].
///
/// This BVH is used to directly or indirectly build any other
/// BVH layout.
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
/// println!("{:?}", bvh.node_count());
/// ```
///
/// /// # Notes
///
/// The lifetime bound is required by [tinybvh](https://github.com/jbikker/tinybvh), which
/// holds to the triangles slice.
pub struct BVH<'a> {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH<'a> {
    pub fn new(vertices: &'a [[f32; 4]]) -> Self {
        let mut inner: cxx::UniquePtr<ffi::BVH> = ffi::new_bvh();
        let primitive_count = vertices.len() as u32 / 3;
        unsafe {
            let ptr = vertices.as_ptr() as *const ffi::bvhvec4;
            inner.pin_mut().Build(ptr, primitive_count);
        }

        BVH {
            inner,
            _phantom: Default::default(),
        }
    }

    pub fn compact(&mut self) {
        self.inner.pin_mut().Compact();
    }

    /// Number of nodes in this BVH.
    ///
    /// # Notes
    ///
    /// - A traversal is required to compute the count
    /// - Root node isn't included in the count
    pub fn node_count(&self) -> u32 {
        self.inner.NodeCount() as u32
    }

    /// Number of primitives for a given node.
    pub fn primitive_count(&self, id: NodeId) -> u32 {
        self.inner.PrimCount(id.0) as u32
    }

    /// SAH cost for a subtree.
    pub fn sah_cost(&self, id: NodeId) -> f32 {
        self.inner.SAHCost(id.0)
    }

    /// BVH nodes.
    ///
    /// Useful to upload to the BVH to the GPU.
    pub fn nodes(&self) -> &[BVHNode] {
        // TODO: Make that safer with cxx
        let ptr = ffi::bvh_nodes(&self.inner) as *const BVHNode;
        let count = ffi::bvh_nodes_count(&self.inner);
        unsafe { from_raw_parts(ptr, count as usize) }
    }
}
