mod cxx_ffi;
mod errors;
mod layouts;
mod traversal;

pub(crate) use cxx_ffi::ffi;
pub use layouts::*;
pub use traversal::*;

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
pub struct Intersection {
    t: f32,
    u: f32,
    v: f32,
    prim: u32,
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Ray {
    origin: [f32; 3],
    padding_0: u32,
    dir: [f32; 3],
    padding_1: u32,
    r_d: [f32; 3],
    padding_2: u32,
    hit: Intersection,
}

//
// Tests
//

#[cfg(test)]
mod tests {
    use crate::{BVHNode, Node4, BVH, BVH4};

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
    fn layout_wald32() {
        let triangles = split_triangles();
        let mut bvh = BVH::new(&triangles);
        assert_eq!(bvh.node_count(), 3);
        assert_eq!(
            bvh.nodes(),
            [
                BVHNode {
                    min: [-2.0, 0.0, -1.0],
                    max: [2.0, 1.0, -1.0],
                    left_first: 2,
                    tri_count: 0
                },
                BVHNode::default(),
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

        bvh.compact();
    }

    #[test]
    fn layout_bvh4() {
        let triangles = split_triangles();
        let bvh: BVH<'_> = BVH::new(&triangles);
        let bvh4: BVH4<'_> = BVH4::new(&bvh);

        let expected = [
            Node4 {
                min: [-2.0, 0.0, -1.0],
                max: [2.0, 1.0, -1.0],
                child: [2, 3, 0, 0],
                child_count: 2,
                ..Default::default()
            },
            Node4::default(),
            Node4 {
                min: [-2.0, 0.0, -1.0],
                max: [-1.0, 1.0, -1.0],
                tri_count: 1,
                ..Default::default()
            },
            Node4 {
                min: [1.0, 0.0, -1.0],
                max: [2.0, 1.0, -1.0],
                tri_count: 1,
                first_tri: 1,
                ..Default::default()
            },
        ];
        assert_eq!(bvh4.nodes(), expected);
    }
}
