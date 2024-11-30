mod ffi;
use std::marker::PhantomData;

use ffi::*;

struct NodeId(u32);

impl NodeId {
    pub fn root() -> Self {
        Self {0: 0}
    }

    pub fn new(id: u32) -> Self {
        Self {0: id}
    }
}

#[enumflags2::bitflags]
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum BVHLayout {
    Wald32Byte,	// Default format, obtained using BVH::Build variants.
    AilaLaine,			// For GPU rendering. Obtained by converting WALD_32BYTE.
    AltSoa,			// For faster CPU rendering. Obtained by converting WALD_32BYTE.
    VERBOSE,			// For BVH optimizing. Obtained by converting WALD_32BYTE.
    BasicBVH4,			// Input for BVH4_GPU conversion. Obtained by converting WALD_32BYTE.
    BVH4GPU,			// For fast GPU rendering. Obtained by converting BASIC_BVH4.
    BVH4Afra,			// For fast CPU rendering. Obtained by converting BASIC_BVH4.
    BasicBVH8,			// Input for CWBVH. Obtained by converting WALD_32BYTE.
    CWBVH				// Fastest GPU rendering. Obtained by converting BASIC_BVH8.
}

struct BVH<'a> {
    inner: cxx::UniquePtr<ffi::ffi::BVH>,
    layout: enumflags2::BitFlags<BVHLayout>,
    _phantom: PhantomData<&'a [f32; 4]>
}

impl<'a> BVH<'a> {
    pub fn new(vertices: &'a [[f32; 4]], primitive_count: u32) -> Self {
        let mut inner: cxx::UniquePtr<ffi::ffi::BVH> = ffi::ffi::new_bvh();
        unsafe {
            let ptr = vertices.as_ptr() as *const ffi::ffi::bvhvec4;
            inner.pin_mut().Build(ptr, primitive_count);
        }

        BVH {
            inner,
            layout: enumflags2::make_bitflags!(BVHLayout::{Wald32Byte}),
            _phantom: Default::default()
        }
    }

    pub fn primitive_count(&self, id: NodeId) -> u32 {
        self.inner.PrimCount(id.0) as u32
    }

    pub fn sah_cost(&self, id: NodeId) -> f32 {
        self.inner.SAHCost(id.0) as f32
    }
}

#[cfg(test)]
mod tests {
    use crate::{NodeId, BVH};

    #[test]
    fn create_bvh() {
        let triangles: Vec<[f32; 4]> = vec![
            [-1.0, 1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],

            [1.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
        ];
        let bvh = BVH::new(&triangles, 2);
        assert_eq!(bvh.primitive_count(NodeId::root()), 2);
    }

}
