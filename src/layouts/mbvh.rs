use std::ops::Deref;

use crate::{cxx_ffi, ffi, wald};

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node {
    aabb_min: [f32; 3],
    first_tri: u32,
    aabb_max: [f32; 3],
    tri_count: u32,
    child: [u32; 8],
    child_count: u32,
    dummy: [u32; ((30 - 8) & 3) + 1],
}

impl Node {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

pub struct BVH {
    pub(crate) inner: cxx::UniquePtr<ffi::MBVH8>,
}

impl BVH {
    pub fn leaf_count(&self, node_index: u32) -> u32 {
        self.inner.LeafCount(node_index)
    }

    pub fn builder<'a>(mut self, original: &'a wald::Builder<'a>) -> Builder<'a> {
        ffi::MBVH8_setBVH(self.inner.pin_mut(), &original.bvh.inner);
        Builder {
            bvh: self,
            original,
        }
    }

    pub fn nodes(&self) -> &[Node] {
        // TODO: Create CWBVH node in tinybvh to avoid that.
        let ptr = ffi::MBVH8_nodes(&self.inner) as *const Node;
        let count = ffi::MBVH8_nodes_count(&self.inner);
        unsafe { std::slice::from_raw_parts(ptr, count as usize) }
    }
}

pub struct Builder<'a> {
    bvh: BVH,
    original: &'a wald::Builder<'a>,
}

impl<'a> Deref for Builder<'a> {
    type Target = BVH;

    fn deref(&self) -> &Self::Target {
        &self.bvh
    }
}

impl<'a> Builder<'a> {
    pub fn new(original: &'a wald::Builder) -> Self {
        let mbvh = BVH {
            inner: ffi::MBVH8_new(),
        };
        let mut builder = mbvh.builder(original);
        builder.convert();
        builder
    }

    pub fn refit(&mut self, node_index: u32) {
        self.bvh.inner.pin_mut().Refit(node_index);
    }

    pub fn convert(&mut self) {
        self.bvh
            .inner
            .pin_mut()
            .ConvertFrom(self.original.bvh.inner.as_ref().unwrap(), true);
    }

    pub fn bvh(self) -> BVH {
        self.bvh
    }
}
