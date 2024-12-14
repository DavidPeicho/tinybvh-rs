use std::{fmt::Debug, marker::PhantomData};

use crate::{ffi, NodeId};

/// "Traditional" 32-bytes BVH node layout, as proposed by Ingo Wald.
///
/// Node layout used by [`BVH`].
///
/// For more information: [tinybvh](https://github.com/jbikker/tinybvh).
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NodeWald {
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

impl NodeWald {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
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
    pub fn new(primitives: &'a [[f32; 4]]) -> Self {
        BVH {
            inner: ffi::new_bvh(),
            _phantom: Default::default(),
        }
        .update(primitives)
    }

    pub fn update(mut self, primitives: &'a [[f32; 4]]) -> Self {
        let primitives = primitives.into();
        self.inner.pin_mut().Build(&primitives);
        // unsafe {
        //     let ptr = primitives.as_ptr() as *const ffi::bvhvec4;
        //     self.inner.pin_mut().Build(ptr, 2);
        // }
        Self {
            inner: self.inner,
            _phantom: PhantomData,
        }
    }

    // Remove unused nodes and reduce the size of the BVH.
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
    pub fn nodes(&self) -> &[NodeWald] {
        ffi::bvh_nodes(&self.inner)
    }
}

impl crate::Intersector for BVH<'_> {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}
