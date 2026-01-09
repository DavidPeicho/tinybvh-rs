use std::ops::Deref;

use crate::{cxx_ffi, ffi, wald};

#[repr(C)]
#[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Node {
    pub aabb_min: [f32; 3],
    pub first_tri: u32,
    pub aabb_max: [f32; 3],
    pub tri_count: u32,
    pub child: [u32; 8],
    pub child_count: u32,
    pub dummy: [u32; ((30 - 8) & 3) + 1],
}

impl Node {
    /// Returns `true` if the node is a leaf.
    pub fn is_leaf(&self) -> bool {
        self.tri_count > 0
    }
}

pub struct BVHData {
    pub(crate) inner: cxx::UniquePtr<ffi::MBVH8>,
    pub(crate) max_primitives_per_leaf: Option<u32>,
}

impl BVHData {
    pub fn leaf_count(&self, node_index: u32) -> u32 {
        self.inner.LeafCount(node_index)
    }

    pub fn builder<'a>(mut self, original: &'a wald::BVH<'a>) -> BVH<'a> {
        ffi::MBVH8_setBVH(self.inner.pin_mut(), &original.bvh.inner);
        BVH {
            bvh: self,
            original,
        }
    }

    pub fn convert<'a>(mut self, original: &'a wald::BVH<'a>) -> BVH<'a> {
        let mut builder = BVH {
            bvh: self,
            original,
        };
        builder.convert();
        builder
    }

    pub fn nodes(&self) -> &[Node] {
        ffi::MBVH8_nodes(&self.inner)
    }
}

pub struct BVH<'a> {
    bvh: BVHData,
    original: &'a wald::BVH<'a>,
}

impl<'a> Deref for BVH<'a> {
    type Target = BVHData;

    fn deref(&self) -> &Self::Target {
        &self.bvh
    }
}

impl<'a> BVH<'a> {
    pub fn new(original: &'a wald::BVH) -> Self {
        let mbvh = BVHData {
            inner: ffi::MBVH8_new(),
            max_primitives_per_leaf: None,
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
        self.bvh.max_primitives_per_leaf = self.original.max_primitives_per_leaf;
    }

    pub fn data(self) -> BVHData {
        self.bvh
    }
}
