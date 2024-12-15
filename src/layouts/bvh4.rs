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

/// BVH4 with layout [`Node4`].
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
pub struct BVH4<'a> {
    inner: cxx::UniquePtr<ffi::BVH4>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH4<'a> {
    pub fn new(primitives: &'a [[f32; 4]]) -> Self {
        Self {
            inner: ffi::new_bvh4(),
            _phantom: PhantomData,
        }
        .build(primitives)
    }

    pub fn nodes(&self) -> &[Node4] {
        ffi::bvh4_nodes(&self.inner)
    }
}
super::impl_bvh!(BVH4, BVH4);
