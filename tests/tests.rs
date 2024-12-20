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
        let mut bvh = BVH::new(&triangles);
        let expected = [
            NodeWald {
                min: [-2.0, 0.0, -1.0],
                max: [2.0, 1.0, -1.0],
                left_first: 2,
                tri_count: 0,
            },
            NodeWald::default(),
            NodeWald {
                min: [-2.0, 0.0, -1.0],
                max: [-1.0, 1.0, -1.0],
                left_first: 0,
                tri_count: 1,
            },
            NodeWald {
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
            let bvh = BVH::new_strided(&positions);
            assert_eq!(bvh.nodes().len(), expected.len());
            assert_eq!(bvh.nodes(), expected);
            assert_eq!(bvh.indices(), [0, 1]);
            test_intersection(&bvh);
        }
    }

    #[test]
    fn layout_bvh4() {
        let triangles = split_triangles();
        let bvh = BVH4::new(&triangles);

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
                first_primitive: 1,
                tri_count: 1,
                ..Default::default()
            },
        ];
        assert_eq!(bvh.nodes().len(), expected.len());
        assert_eq!(bvh.nodes(), expected);

        assert_eq!(bvh.indices(), [0, 1]);
        test_intersection(&bvh);
    }

    #[test]
    fn layout_cwbvh() {
        let primitives = split_triangles();
        let bvh = CWBVH::new(&primitives);
        assert_eq!(bvh.nodes().len(), 5);

        assert_eq!(
            bvh.primitives(),
            [
                PrimitiveCWBVH {
                    vertex_0: [-2.0, 1.0, -1.0],
                    vertex_1: [-1.0, 1.0, -1.0],
                    vertex_2: [-2.0, 0.0, -1.0],
                    original_primitive: 0,
                    ..Default::default()
                },
                PrimitiveCWBVH {
                    vertex_0: [2.0, 1.0, -1.0],
                    vertex_1: [2.0, 0.0, -1.0],
                    vertex_2: [1.0, 0.0, -1.0],
                    original_primitive: 1,
                    ..Default::default()
                }
            ]
        );
    }

    #[test]
    fn capture() {
        let mut triangles = split_triangles();
        let bvh = BVH::new(&triangles);
        assert_relative_eq!(bvh.nodes()[0].min[0], -2.0);

        let capture = bvh.capture();
        triangles[0][0] = -5.0;
        let bvh: BVH<'_> = BVH::from_capture(capture, &triangles);
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
        let _ = BVH::new(&primitives);
    }
}
