use std::{fmt::Debug, marker::PhantomData};

use crate::{ffi, Intersector, Ray};

use super::impl_bvh_layout;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node4 {
    pub min: [f32; 3],
    pub first_tri: u32,
    pub max: [f32; 3],
    pub tri_count: u32,
    pub child: [u32; 4],
    pub child_count: u32,
    pub padding: [u32; 3],
}

impl Node4 {
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

/// BVH
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

impl_bvh_layout!(BVH4);

impl Intersector for BVH4<'_> {
    fn intersect(&self, ray: &mut Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}
