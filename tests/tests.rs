#[cfg(test)]
mod tests {
    use core::f32;

    use approx::assert_relative_eq;
    use tinybvh_rs::*;

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

    #[cfg(feature = "strided")]
    #[repr(C)]
    #[derive(Clone, Copy, Default, Debug, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
    struct Vertex {
        normal: [f32; 3],
        position: [f32; 4],
        uv: [f32; 2],
    }
    #[test]
    fn layout_wald32() {
        let triangles = split_triangles();
        let mut bvh = wald::BVH::new(&triangles);
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

        #[cfg(feature = "strided")]
        {
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
            let bvh = wald::BVH::new_strided(&positions);
            assert_eq!(bvh.nodes().len(), expected.len());
            assert_eq!(bvh.nodes(), expected);
            assert_eq!(bvh.indices(), [0, 1]);
            test_intersection(&bvh);
        }
    }

    #[test]
    fn layout_cwbvh() {
        let primitives = split_triangles();
        let bvh = cwbvh::BVH::new(&primitives);
        assert_eq!(bvh.nodes().len(), 1);
        assert_eq!(bvh.nodes()[0].primitives().collect::<Vec<u32>>(), [0, 1]);

        assert_eq!(
            bvh.primitives(),
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
    }

    #[test]
    fn capture() {
        let mut triangles = split_triangles();
        let bvh = wald::BVH::new(&triangles);
        assert_relative_eq!(bvh.nodes()[0].min[0], -2.0);

        let capture = bvh.capture();
        triangles[0][0] = -5.0;
        let bvh = wald::BVH::from_capture(capture, &triangles);
        assert_relative_eq!(bvh.nodes()[0].min[0], -5.0);
    }

    #[test]
    #[should_panic]
    fn panic_non_triangulated() {
        let primitives = [
            [1.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        ];
        let _ = wald::BVH::new(&primitives);
    }
}
