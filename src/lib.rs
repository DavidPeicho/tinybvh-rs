mod errors;
mod ffi;
mod node;

pub use node::*;
use std::{marker::PhantomData, slice::from_raw_parts};

#[enumflags2::bitflags]
#[repr(u16)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BVHLayoutType {
    Wald32Byte, // Default format, obtained using BVH::Build variants.
    AilaLaine,  // For GPU rendering. Obtained by converting WALD_32BYTE.
    AltSoa,     // For faster CPU rendering. Obtained by converting WALD_32BYTE.
    Verbose,    // For BVH optimizing. Obtained by converting WALD_32BYTE.
    BasicBVH4,  // Input for BVH4_GPU conversion. Obtained by converting WALD_32BYTE.
    BVH4GPU,    // For fast GPU rendering. Obtained by converting BASIC_BVH4.
    BVH4Afra,   // For fast CPU rendering. Obtained by converting BASIC_BVH4.
    BasicBVH8,  // Input for CWBVH. Obtained by converting WALD_32BYTE.
    CWBVH,      // Fastest GPU rendering. Obtained by converting BASIC_BVH8.
}

impl Into<ffi::ffi::BVHLayout> for BVHLayoutType {
    fn into(self) -> ffi::ffi::BVHLayout {
        match self {
            BVHLayoutType::Wald32Byte => ffi::ffi::BVHLayout::WALD_32BYTE,
            BVHLayoutType::AilaLaine => ffi::ffi::BVHLayout::AILA_LAINE,
            BVHLayoutType::AltSoa => ffi::ffi::BVHLayout::ALT_SOA,
            BVHLayoutType::Verbose => ffi::ffi::BVHLayout::VERBOSE,
            BVHLayoutType::BasicBVH4 => ffi::ffi::BVHLayout::BASIC_BVH4,
            BVHLayoutType::BVH4GPU => ffi::ffi::BVHLayout::BVH4_GPU,
            BVHLayoutType::BVH4Afra => ffi::ffi::BVHLayout::BVH4_AFRA,
            BVHLayoutType::BasicBVH8 => ffi::ffi::BVHLayout::BASIC_BVH8,
            BVHLayoutType::CWBVH => ffi::ffi::BVHLayout::CWBVH,
        }
    }
}

pub struct BVH<'a> {
    inner: cxx::UniquePtr<ffi::ffi::BVH>,
    layout: enumflags2::BitFlags<BVHLayoutType>,
    _phantom: PhantomData<&'a [f32; 4]>,
}

impl<'a> BVH<'a> {
    pub fn new(vertices: &'a [[f32; 4]]) -> Self {
        let mut inner: cxx::UniquePtr<ffi::ffi::BVH> = ffi::ffi::new_bvh();
        let primitive_count = vertices.len() as u32 / 3;
        unsafe {
            let ptr = vertices.as_ptr() as *const ffi::ffi::bvhvec4;
            inner.pin_mut().Build(ptr, primitive_count);
        }

        BVH {
            inner,
            layout: enumflags2::make_bitflags!(BVHLayoutType::{Wald32Byte}),
            _phantom: Default::default(),
        }
    }

    pub fn compact(&mut self, layout: BVHLayoutType) -> Result<(), errors::BVHCompactError> {
        if layout != BVHLayoutType::Wald32Byte && layout != BVHLayoutType::Verbose {
            return Err(errors::BVHCompactError::UnsupportedLayout(layout));
        }
        self.validate_layout(layout)?;
        self.inner.pin_mut().Compact(layout.into());
        Ok(())
    }

    pub fn convert(&mut self, to_layout: BVHLayoutType) -> Result<(), errors::MissingLayout> {
        let from_layout = match to_layout {
            BVHLayoutType::AilaLaine
            | BVHLayoutType::AltSoa
            | BVHLayoutType::Verbose
            | BVHLayoutType::BasicBVH4
            | BVHLayoutType::BasicBVH8 => BVHLayoutType::Wald32Byte,
            BVHLayoutType::BVH4GPU | BVHLayoutType::BVH4Afra => BVHLayoutType::BasicBVH4,
            BVHLayoutType::CWBVH => BVHLayoutType::BasicBVH8,
            BVHLayoutType::Wald32Byte => BVHLayoutType::Verbose,
        };
        self.validate_layout(from_layout)?;
        self.inner
            .pin_mut()
            .Convert(from_layout.into(), to_layout.into(), false);
        Ok(())
    }

    /// Doesn't include the **root** node
    pub fn node_count(&self, layout: BVHLayoutType) -> Option<u32> {
        match self.validate_layout(layout) {
            Ok(_) => Some(self.inner.NodeCount(layout.into()) as u32),
            Err(_) => None,
        }
    }

    pub fn primitive_count(&self, id: NodeId) -> u32 {
        self.inner.PrimCount(id.0) as u32
    }

    pub fn sah_cost(&self, id: NodeId) -> f32 {
        self.inner.SAHCost(id.0) as f32
    }

    pub fn layout(&self) -> enumflags2::BitFlags<BVHLayoutType> {
        self.layout
    }

    pub fn as_wald32(&self) -> &[BVHNode] {
        let ptr = ffi::ffi::bvh_nodes(&self.inner) as *const BVHNode;
        let count = ffi::ffi::bvh_nodes_count(&self.inner);
        unsafe { from_raw_parts(ptr, count as usize) }
    }

    fn validate_layout(&self, layout: BVHLayoutType) -> Result<(), errors::MissingLayout> {
        match self.layout.contains(layout) {
            true => Ok(()),
            false => Err(errors::MissingLayout::new(layout)),
        }
    }
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::{BVHLayoutType, BVHNode, BVH};
    use enumflags2::make_bitflags;

    const CUBE_INDICES: [u16; 36] = [
        0, 1, 2, 2, 3, 0, // top
        4, 5, 6, 6, 7, 4, // bottom
        8, 9, 10, 10, 11, 8, // right
        12, 13, 14, 14, 15, 12, // left
        16, 17, 18, 18, 19, 16, // front
        20, 21, 22, 22, 23, 20, // back
    ];
    const CUBE_POSITIONS: [[f32; 3]; 24] = [
        // top (0, 0, 1)
        [-1.0, -1.0, 1.0],
        [1.0, -1.0, 1.0],
        [1.0, 1.0, 1.0],
        [-1.0, 1.0, 1.0],
        // bottom (0, 0, -1.0)
        [-1.0, 1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, -1.0, -1.0],
        [-1.0, -1.0, -1.0],
        // right (1.0, 0, 0)
        [1.0, -1.0, -1.0],
        [1.0, 1.0, -1.0],
        [1.0, 1.0, 1.0],
        [1.0, -1.0, 1.0],
        // left (-1.0, 0, 0)
        [-1.0, -1.0, 1.0],
        [-1.0, 1.0, 1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, -1.0, -1.0],
        // front (0, 1.0, 0)
        [1.0, 1.0, -1.0],
        [-1.0, 1.0, -1.0],
        [-1.0, 1.0, 1.0],
        [1.0, 1.0, 1.0],
        // back (0, -1.0, 0)
        [1.0, -1.0, 1.0],
        [-1.0, -1.0, 1.0],
        [-1.0, -1.0, -1.0],
        [1.0, -1.0, -1.0],
    ];

    fn plane() -> Vec<[f32; 4]> {
        vec![
            [-1.0, 1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
            [1.0, 1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0, 0.0],
        ]
    }

    fn split_triangles() -> Vec<[f32; 4]> {
        vec![
            [-2.0, 1.0, -1.0, 0.0],
            [-1.0, 1.0, -1.0, 0.0],
            [-2.0, 0.0, -1.0, 0.0],
            [2.0, 1.0, -1.0, 0.0],
            [2.0, 0.0, -1.0, 0.0],
            [1.0, 0.0, -1.0, 0.0],
        ]
    }

    fn cube() -> Vec<[f32; 4]> {
        let mut triangles: Vec<[f32; 4]> = Vec::with_capacity(CUBE_INDICES.len() / 3);
        for index in CUBE_INDICES {
            let pos = CUBE_POSITIONS[index as usize];
            triangles.push([pos[0], pos[1], pos[2], 0.0]);
        }
        triangles
    }

    #[test]
    fn layout_info() {
        let triangles: Vec<[f32; 4]> = plane();
        let bvh = BVH::new(&triangles);

        assert_eq!(bvh.layout(), make_bitflags!(BVHLayoutType::{Wald32Byte}));

        assert_eq!(bvh.node_count(BVHLayoutType::Wald32Byte), Some(1));
        assert_eq!(bvh.node_count(BVHLayoutType::AilaLaine), None);
        assert_eq!(bvh.node_count(BVHLayoutType::AltSoa), None);
        assert_eq!(bvh.node_count(BVHLayoutType::Verbose), None);
        assert_eq!(bvh.node_count(BVHLayoutType::BasicBVH4), None);
        assert_eq!(bvh.node_count(BVHLayoutType::BVH4GPU), None);
        assert_eq!(bvh.node_count(BVHLayoutType::BVH4Afra), None);
        assert_eq!(bvh.node_count(BVHLayoutType::BasicBVH8), None);
        assert_eq!(bvh.node_count(BVHLayoutType::CWBVH), None);
    }

    #[test]
    fn layout_wald32() {
        let triangles = split_triangles();
        let bvh = BVH::new(&triangles);
        assert_eq!(bvh.node_count(BVHLayoutType::Wald32Byte), Some(3));

        let nodes = bvh.as_wald32();
        println!("{:?}", nodes);

        assert_eq!(
            nodes,
            [
                BVHNode {
                    min: [-2.0, 0.0, -1.0],
                    max: [2.0, 1.0, -1.0],
                    left_first: 2,
                    tri_count: 0
                },
                BVHNode {
                    min: [0.0, 0.0, 0.0],
                    max: [0.0, 0.0, 0.0],
                    left_first: 0,
                    tri_count: 0
                },
                BVHNode {
                    min: [-2.0, 0.0, -1.0],
                    max: [-1.0, 1.0, -1.0],
                    left_first: 0,
                    tri_count: 1
                },
                BVHNode {
                    min: [1.0, 0.0, -1.0],
                    max: [2.0, 1.0, -1.0],
                    left_first: 1,
                    tri_count: 1
                },
            ]
        );
        assert!(!nodes[0].is_leaf());
    }

    #[test]
    fn compact() {
        let triangles = cube();
        let mut bvh = BVH::new(&triangles);

        assert_eq!(bvh.node_count(BVHLayoutType::Wald32Byte), Some(11));
        bvh.compact(BVHLayoutType::Wald32Byte).unwrap();
        assert_eq!(bvh.node_count(BVHLayoutType::Wald32Byte), Some(11));
    }

    #[test]
    fn convert() {
        let triangles = cube();
        let mut bvh = BVH::new(&triangles);

        for layout in [
            BVHLayoutType::AilaLaine,
            BVHLayoutType::AltSoa,
            BVHLayoutType::Verbose,
            BVHLayoutType::BasicBVH4,
            BVHLayoutType::BasicBVH8,
        ] {
            bvh.convert(layout).unwrap();
        }
    }
}
