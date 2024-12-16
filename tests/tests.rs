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

    fn sphere() -> Vec<[f32; 4]> {
        let radius = 10.0;
        let width_segments = 32;
        let height_segments = 16;
        let phi_length = f32::consts::PI * 2.0;
        let mut vertices = Vec::new();
        for iy in 0..height_segments {
            let v = iy as f32 / height_segments as f32;
            for ix in 0..width_segments {
                let u = ix as f32 / width_segments as f32;
                let vertex: [f32; 4] = [
                    -radius * f32::cos(u * phi_length) * f32::sin(v * f32::consts::PI),
                    radius * f32::cos(v * f32::consts::PI),
                    radius * f32::sin(u * phi_length) * f32::sin(v * f32::consts::PI),
                    0.0 as f32,
                ];
                vertices.push(vertex);
            }
        }
        vertices
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
    }

    #[test]
    fn layout_bvh4() {
        let triangles = split_triangles();
        let bvh = BVH4::new(&triangles);
        // Not checking for full layout, since the number of nodes vary
        // per platform
        assert_eq!(bvh.indices(), [0, 1]);
        test_intersection(&bvh);
    }

    #[test]
    fn layout_cwbvh() {
        let primitives = split_triangles();
        let bvh = CWBVH::new(&primitives);
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
}
