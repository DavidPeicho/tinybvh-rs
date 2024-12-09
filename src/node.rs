use std::fmt::Debug;

pub struct NodeId(pub u32);

impl NodeId {
    pub fn root() -> Self {
        Self(0)
    }

    pub fn new(id: u32) -> Self {
        Self(id)
    }
}

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
