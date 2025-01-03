use crate::ffi;
use std::{fmt::Debug, marker::PhantomData};

/// "Traditional" 32-bytes BVH node layout, as proposed by Ingo Wald.
///
/// Node layout used by [`BVH`].
#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node {
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

impl Node {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

/// BVH with node layout [`Node`].
///
/// # Examples
///
/// ```
/// use tinybvh_rs::wald;
///
/// let triangles = vec![
///     [-1.0, 1.0, 0.0, 0.0],
///     [1.0, 1.0, 0.0, 0.0],
///     [-1.0, 0.0, 0.0, 0.0]
/// ];
/// let bvh = wald::BVH::new(&triangles);
/// ```
pub struct BVH<'a> {
    inner: cxx::UniquePtr<ffi::BVH>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH<'a> {
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
    pub fn nodes(&self) -> &[Node] {
        ffi::BVH_nodes(&self.inner)
    }

    /// BVH indices.
    ///
    /// Map from primitive index to first vertex index.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for i in 0..node.tri_count {
    ///     let vertex_start = bvh.indices()[node.left_first + i] * 3;
    ///     let vertex = [
    ///         primitives[vertex_start],
    ///         primitives[vertex_start + 1],
    ///         primitives[vertex_start + 2]
    ///     ];
    ///     println!("Vertex {:?}", vertex);
    /// }
    /// ```
    pub fn indices(&self) -> &[u32] {
        ffi::BVH_indices(&self.inner)
    }

    pub fn new_internal() -> Self {
        Self {
            inner: ffi::BVH_new(),
            _phantom: PhantomData,
        }
    }
}
super::impl_bvh!(BVH, BVH);

impl crate::Intersector for BVH<'_> {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}
