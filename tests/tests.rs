#[cfg(test)]
mod tests {
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

        // Intersection testing

        let mut ray: Ray = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
        assert_eq!(bvh4.intersect(&mut ray), 1);
        assert_relative_eq!(ray.hit.t, INFINITE);

        let mut ray: Ray = Ray::new([-1.5, 0.5, 0.0], [0.0, 0.0, -1.0]);
        assert_eq!(bvh4.intersect(&mut ray), 2);
        assert_relative_eq!(ray.hit.t, 1.0);
        assert_eq!(ray.hit.prim, 0);

        let mut ray: Ray = Ray::new([1.5, 0.45, 0.0], [0.0, 0.0, -1.0]);
        assert_eq!(bvh4.intersect(&mut ray), 2);
        assert_relative_eq!(ray.hit.t, 1.0);
        assert_eq!(ray.hit.prim, 1);
    }
}
