use std::{fmt::Debug, marker::PhantomData, slice::from_raw_parts};

use crate::{ffi, BVH};

#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
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

// impl PartialEq for Node4 {
//     fn eq(&self, other: &Self) -> bool {
//         if self.first_tri != other.first_tri
//             || self.tri_count != other.tri_count
//             || self.child_count != other.child_count
//         {
//             return false;
//         }
//         self.child.eq(other.child) &&
//     }
// }

impl Node4 {
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

impl Debug for Node4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node4")
            .field("min", &self.min)
            .field("max", &self.max)
            .field("tri_count", &self.tri_count)
            .field("child", &self.child)
            .field("child_count", &self.child_count)
            .field("padding", &self.padding)
            .finish()
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
