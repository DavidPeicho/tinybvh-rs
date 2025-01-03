# tinybvh-rs

Rust wrapper for [tinybvh](https://github.com/jbikker/tinybvh).

## Features

Provides BVH (Bounding Volume Hierarchy) construction and intersection:
- Construction: [`BVH`], [`BVH4`], [`CWBVH`]
- Intersection

For more information about each layout: [tinybvh](https://github.com/jbikker/tinybvh).

## Examples

### BVH Wald

```rust
use tinybvh_rs::{Intersector, Ray};

let primitives = vec![
    [-2.0, 1.0, -1.0, 0.0],    //
    [-1.0, 1.0, -1.0, 0.0],    // Left triangle
    [-2.0, 0.0, -1.0, 0.0],    //

    [2.0, 1.0, -1.0, 0.0],     //
    [2.0, 0.0, -1.0, 0.0],     // Right triangle
    [1.0, 0.0, -1.0, 0.0],     //
];

let bvh = wald::BVH::new(&primitives);

// No intersection, ray pass between the primitives
let mut ray = Ray::new([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]);
bvh.intersect(&mut ray);
println!("Hit distance: {}", ray.hit.t); // 1e30

// Intersects left primitive
let mut ray = Ray::new([-1.5, 0.5, 0.0], [0.0, 0.0, -1.0]);
bvh.intersect(&mut ray);
println!("Hit distance & primtive: {} / {}", ray.hit.t, ray.hit.prim); // 1.0 / 0

// Intersects right primitive
let mut ray = Ray::new([1.5, 0.45, 0.0], [0.0, 0.0, -1.0]);
bvh.intersect(&mut ray);
println!("Hit distance & primtive: {} / {}", ray.hit.t, ray.hit.prim); // 1.0 / 1
```

### Strided

If the vertices position are strided (located in a `Vertex` struct for instance),
you can enable the `strided` feature and use:

```rust
use tinybvh_rs::{Intersector, Ray};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 4],
    normal: [f32; 3],
}

let vertices = [
    Vertex {
        position: [-1.0, 1.0, -1.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
    Vertex {
        position: [-0.5, 1.0, -1.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
    Vertex {
        position: [-1.0, 0.0, -1.0, 0.0],
        normal: [0.0, 0.0, 1.0]
    },
];
let positions = pas::slice_attr!(vertices, [0].position);
let bvh = wald::BVH::new(&positions);
```
