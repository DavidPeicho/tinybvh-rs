use std::{fmt::Debug, marker::PhantomData, slice::from_raw_parts};

use crate::{ffi, NodeId};

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct BVHNode {
    pub min: [f32; 3],
    pub left_first: u32,
    pub max: [f32; 3],
    pub tri_count: u32,
}

impl BVHNode {
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

    /// Doesn't include the **root** node
    pub fn node_count(&self) -> u32 {
        self.inner.NodeCount() as u32
    }

    pub fn primitive_count(&self, id: NodeId) -> u32 {
        self.inner.PrimCount(id.0) as u32
    }

    pub fn sah_cost(&self, id: NodeId) -> f32 {
        self.inner.SAHCost(id.0)
    }

    pub fn nodes(&self) -> &[BVHNode] {
        // TODO: Make that safer with cxx
        let ptr = ffi::bvh_nodes(&self.inner) as *const BVHNode;
        let count = ffi::bvh_nodes_count(&self.inner);
        unsafe { from_raw_parts(ptr, count as usize) }
    }
}
