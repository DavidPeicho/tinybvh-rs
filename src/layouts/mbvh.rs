use std::fmt::Debug;

use crate::{ffi, layouts::impl_bvh_deref, wald};

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

    pub fn bvh<'a>(mut self, original: &'a wald::BVH<'a>) -> BVH<'a> {
        ffi::MBVH8_setBVH(self.inner.pin_mut(), &original.bvh.inner);
        BVH {
            bvh: self,
            original,
        }
    }

    pub fn convert<'a>(mut self, original: &'a wald::BVH<'a>) -> BVH<'a> {
        BVH {
            bvh: self,
            original,
        }
        .convert(original)
    }

    pub fn nodes(&self) -> &[Node] {
        ffi::MBVH8_nodes(&self.inner)
    }
}

pub struct BVH<'a> {
    bvh: BVHData,
    original: &'a wald::BVH<'a>,
}
impl_bvh_deref!(BVH<'a>, BVHData);

impl<'a> BVH<'a> {
    pub fn new(original: &'a wald::BVH) -> Self {
        let data = BVHData {
            inner: ffi::MBVH8_new(),
            max_primitives_per_leaf: None,
        };
        data.convert(original)
    }

    pub fn convert<'b>(mut self, original: &'b wald::BVH) -> BVH<'b> {
        let mut bvh = self.bvh;
        bvh.inner
            .pin_mut()
            .ConvertFrom(original.bvh.inner.as_ref().unwrap(), true);
        bvh.max_primitives_per_leaf = original.max_primitives_per_leaf;
        BVH { bvh, original }
    }

    pub fn refit(&mut self, node_index: u32) {
        self.bvh.inner.pin_mut().Refit(node_index);
    }

    pub fn data(self) -> BVHData {
        self.bvh
    }
}
