use std::{fmt::Debug, marker::PhantomData, pin::Pin, slice::from_raw_parts};

use crate::{ffi, Intersector, Ray, BVH};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
// 4-wide (aka 'shallow') node
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

pub struct BVH4<'a> {
    inner: cxx::UniquePtr<ffi::BVH4>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH4<'a> {
    pub fn new(bvh: &BVH) -> Self {
        let mut inner: cxx::UniquePtr<ffi::BVH4> = ffi::new_bvh4();
        inner.pin_mut().ConvertFrom(&bvh.inner);
        Self {
            inner,
            _phantom: PhantomData,
        }
    }

    pub fn update(mut self, bvh: BVH<'a>) -> Self {
        self.inner.pin_mut().ConvertFrom(&bvh.inner);
        Self {
            inner: self.inner,
            _phantom: PhantomData,
        }
    }

    pub fn nodes(&self) -> &[Node4] {
        // TODO: Make that safer with cxx
        let ptr = ffi::bvh4_nodes(&self.inner) as *const Node4;
        let count = ffi::bvh4_nodes_count(&self.inner);
        unsafe { from_raw_parts(ptr, count as usize) }
    }
}

impl Intersector for BVH4<'_> {
    fn intersect(&self, ray: &mut Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}
