#[cfg(test)]
mod tests {
    use core::f32;
    use std::fmt::Debug;

    use approx::assert_relative_eq;
    use tinybvh_rs::*;

    fn split_triangles(scale: f32) -> Vec<[f32; 4]> {
        let double = 2.0 * scale;
        vec![
            [-double, scale, -1.0, 0.0],
            [-scale, scale, -1.0, 0.0],
            [-double, 0.0, -1.0, 0.0],
            [double, scale, -1.0, 0.0],
            [double, 0.0, -1.0, 0.0],
            [scale, 0.0, -1.0, 0.0],
        ]
    }

    fn test_intersection<B: Intersector>(bvh: &B) {
        let mut ray: Ray = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
        assert_eq!(bvh.intersect(&mut ray), 1);
        assert_relative_eq!(ray.hit.t, INFINITE);

        let mut ray: Ray = Ray::new([-1.5, 0.5, 0.0], [0.0, 0.0, -1.0]);
        bvh.intersect(&mut ray);
        assert_relative_eq!(ray.hit.t, 1.0);
        assert_eq!(ray.hit.prim, 0);

        let mut ray: Ray = Ray::new([1.5, 0.45, 0.0], [0.0, 0.0, -1.0]);
        bvh.intersect(&mut ray);
        assert_relative_eq!(ray.hit.t, 1.0);
        assert_eq!(ray.hit.prim, 1);
    }

    #[repr(C)]
    #[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
    struct Vertex {
        normal: [f32; 3],
        position: [f32; 4],
        uv: [f32; 2],
    }
    #[test]
    fn layout_wald32() {
        let triangles = split_triangles(1.0);
        let mut bvh = wald::BVH::new(triangles.as_slice().into());
        let expected = [
            wald::Node {
                min: [-2.0, 0.0, -1.0],
                max: [2.0, 1.0, -1.0],
                left_first: 2,
                tri_count: 0,
            },
            wald::Node::default(),
            wald::Node {
                min: [-2.0, 0.0, -1.0],
                max: [-1.0, 1.0, -1.0],
                left_first: 0,
                tri_count: 1,
            },
            wald::Node {
                min: [1.0, 0.0, -1.0],
                max: [2.0, 1.0, -1.0],
                left_first: 1,
                tri_count: 1,
            },
        ];
        assert_eq!(bvh.nodes().len(), expected.len());
        assert_eq!(bvh.nodes(), expected);
        assert_eq!(bvh.indices(), [0, 1]);
        test_intersection(&bvh);

        bvh.compact();
        test_intersection(&bvh);

        use pas::slice_attr;
        let primitives = [
            Vertex {
                position: [-2.0, 1.0, -1.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [-1.0, 1.0, -1.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [-2.0, 0.0, -1.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [2.0, 1.0, -1.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [2.0, 0.0, -1.0, 0.0],
                ..Default::default()
            },
            Vertex {
                position: [1.0, 0.0, -1.0, 0.0],
                ..Default::default()
            },
        ];
        let positions = slice_attr!(primitives, [0].position);
        let bvh = bvh.build_hq(positions);
        assert_eq!(bvh.nodes().len(), expected.len());
        assert_eq!(bvh.nodes(), expected);
        assert_eq!(bvh.indices(), [0, 1]);
        test_intersection(&bvh);
    }

    #[test]
    fn layout_mbvh8() {
        let primitives = split_triangles(1.0);
        let bvh = wald::BVH::new(primitives.as_slice().into());
        let mbvh = mbvh::BVH::new(&bvh);
        assert_eq!(mbvh.leaf_count(0), 2);
        assert_eq!(mbvh.nodes().len(), 4);

        let leaves: Vec<bool> = mbvh.nodes().iter().map(|n| n.is_leaf()).collect();
        assert_eq!(leaves, [false, false, true, true]);
        assert_eq!(mbvh.nodes()[0].aabb_min, [-2.0, 0.0, -1.0]);
        assert_eq!(mbvh.nodes()[0].aabb_max, [2.0, 1.0, -1.0]);

        // Update vertices only, doesn't change node bounds

        let mbvh = mbvh.data();
        let bvh = bvh.data();
        let other_positions: Vec<[f32; 4]> = split_triangles(2.0);
        let bvh = bvh.builder(other_positions.as_slice().into());
        let mbvh = mbvh.convert(&bvh);
        assert_eq!(mbvh.nodes().len(), 4);
        assert_eq!(mbvh.nodes()[0].aabb_min, [-2.0, 0.0, -1.0]);
        assert_eq!(mbvh.nodes()[0].aabb_max, [2.0, 1.0, -1.0]);

        // Update mbvh's original

        let other_positions = split_triangles(3.0);
        let bvh2: wald::BVH<'_> = wald::BVH::new(other_positions.as_slice().into());
        let mbvh: mbvh::BVH<'_> = mbvh.data().convert(&bvh2);
        assert_eq!(mbvh.nodes().len(), 4);
        assert_eq!(mbvh.nodes()[0].aabb_min, [-6.0, 0.0, -1.0]);
        assert_eq!(mbvh.nodes()[0].aabb_max, [6.0, 3.0, -1.0]);

        let mbvh = mbvh.data();
        let bvh = bvh.data();
        let primitives = split_triangles(10.0);
        let bvh = bvh.builder(primitives.as_slice().into());
        let mut mbvh = mbvh.builder(&bvh);
        mbvh.refit(0);

        assert_eq!(mbvh.nodes()[0].aabb_min, [-20.0, 0.0, -1.0]);
        assert_eq!(mbvh.nodes()[0].aabb_max, [20.0, 10.0, -1.0]);
    }

    #[test]
    fn layout_cwbvh() {
        let primitives = split_triangles(1.0);

        let mut bvh = wald::BVH::new(primitives.as_slice().into());
        bvh.split_leaves(3);
        let mbvh = mbvh::BVH::new(&bvh);

        let mut cwbvh = cwbvh::BVH::new(&mbvh).unwrap();
        assert_eq!(cwbvh.nodes().len(), 1);
        assert_eq!(cwbvh.nodes()[0].primitives().collect::<Vec<u32>>(), [0, 1]);
        assert_eq!(
            cwbvh.primitives(),
            [
                cwbvh::Primitive {
                    vertex_0: [-2.0, 1.0, -1.0],
                    edge_1: [0.0, -1.0, 0.0],
                    edge_2: [1.0, 0.0, 0.0],
                    original_primitive: 0,
                    ..Default::default()
                },
                cwbvh::Primitive {
                    vertex_0: [2.0, 1.0, -1.0],
                    edge_1: [-1.0, -1.0, 0.0],
                    edge_2: [0.0, -1.0, 0.0],
                    original_primitive: 1,
                    ..Default::default()
                }
            ]
        );

        // Update mbvh

        let other_positions = split_triangles(2.0);
        let mut bvh: wald::BVH<'_> = wald::BVH::new(other_positions.as_slice().into());
        bvh.split_leaves(3);
        let mbvh: mbvh::BVH<'_> = mbvh.data().convert(&bvh);
        cwbvh.convert(&mbvh);
        assert_eq!(
            cwbvh.primitives(),
            [
                cwbvh::Primitive {
                    vertex_0: [-4.0, 2.0, -1.0],
                    edge_1: [0.0, -2.0, 0.0],
                    edge_2: [2.0, 0.0, 0.0],
                    original_primitive: 0,
                    ..Default::default()
                },
                cwbvh::Primitive {
                    vertex_0: [4.0, 2.0, -1.0],
                    edge_1: [-2.0, -2.0, 0.0],
                    edge_2: [0.0, -2.0, 0.0],
                    original_primitive: 1,
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn layout_bvh8_cpu() {
        let primitives = split_triangles(1.0);

        let bvh = wald::BVH::new(primitives.as_slice().into());
        let mbvh = mbvh::BVH::new(&bvh).data();
        assert_eq!(mbvh.leaf_count(0), 2);

        #[cfg(target_feature = "avx2")]
        {
            let bvh8 = bvh8_cpu::BVH::new(&mbvh).data();
            test_intersection(&bvh8);
        }
        test_intersection(&bvh);
    }

    // #[test]
    // #[should_panic]
    // fn builder_capture_error() {
    //     let bvh = {
    //         let primitives = split_triangles();
    //         let bvh = bvh8_cpu::BVH::new(primitives.as_slice());
    //         bvh.data()
    //     };

    //     let primitives = vec![
    //         [-2.0, 1.0, -1.0, 0.0],
    //         [-1.0, 1.0, -1.0, 0.0],
    //         [-2.0, 0.0, -1.0, 0.0],
    //     ];
    //     bvh.builder(primitives.as_slice());
    // }

    // #[test]
    // #[should_panic]
    // fn panic_non_triangulated() {
    //     let primitives = [
    //         [1.0, 0.0, 0.0, 0.0],
    //         [1.0, 0.0, 0.0, 0.0],
    //         [1.0, 0.0, 0.0, 0.0],
    //         [1.0, 0.0, 0.0, 0.0],
    //     ];
    //     let _ = wald::BVH::new(primitives.as_slice().into());
    // }
}
