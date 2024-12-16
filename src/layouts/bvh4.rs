use crate::ffi;
use std::{fmt::Debug, marker::PhantomData};

/// 4-wide (A.K.A 'shallow') BVH layout.
///
/// Node layout used by [`BVH4`].
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node4 {
    /// AABB min position.
    pub min: [f32; 3],
    pub first_tri: u32,
    /// AABB max position.
    pub max: [f32; 3],
    /// If the node is a leaf, number of triangles in the node.
    /// `0` otherwise.
    pub tri_count: u32,
    pub child: [u32; 4],
    pub child_count: u32,
    pub padding: [u32; 3],
}

impl Node4 {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

/// BVH4 with node layout [`Node4`].
///
/// # Examples
///
/// ```
/// use tinybvh_rs::BVH4;
///
/// let triangles = vec![
///     [-1.0, 1.0, 0.0, 0.0],
///     [1.0, 1.0, 0.0, 0.0],
///     [-1.0, 0.0, 0.0, 0.0]
/// ];
/// let bvh = BVH4::new(&triangles);
/// ```
///
/// # Notes
///
/// This layout relies on another layout, building it implicitly:
/// - Build a [`crate::BVH`]
pub struct BVH4<'a> {
    inner: cxx::UniquePtr<ffi::BVH4>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH4<'a> {
    pub fn new(primitives: &'a [[f32; 4]]) -> Self {
        Self {
            inner: ffi::BVH4_new(),
            _phantom: PhantomData,
        }
        .build(primitives)
    }

    /// Node hierarchy.
    pub fn nodes(&self) -> &[Node4] {
        ffi::BVH4_nodes(&self.inner)
    }

    /// BVH indices.
    ///
    /// Map from primitive index to first vertex index.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// for i in 0..node.tri_count {
    ///     let vertex_start = bvh.indices()[node.first_tri + i] * 3;
    ///     let vertex = [
    ///         primitives[vertex_start],
    ///         primitives[vertex_start + 1],
    ///         primitives[vertex_start + 2]
    ///     ];
    ///     println!("Vertex {:?}", vertex);
    /// }
    /// ```
    pub fn indices(&self) -> &[u32] {
        ffi::BVH4_indices(&self.inner)
    }
}
super::impl_bvh!(BVH4, BVH4);

impl crate::Intersector for BVH4<'_> {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}
