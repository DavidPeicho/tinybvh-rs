use core::f32;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Intersection {
    pub t: f32,
    pub u: f32,
    pub v: f32,
    pub prim: u32,
}

impl Intersection {
    pub fn new() -> Self {
        Self {
            t: f32::MAX,
            ..Default::default()
        }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Ray {
    pub origin: [f32; 3],
    pub padding_0: u32,
    pub dir: [f32; 3],
    pub padding_1: u32,
    pub r_d: [f32; 3],
    pub padding_2: u32,
    pub hit: Intersection,
}

impl Ray {
    pub fn new(origin: [f32; 3], dir: [f32; 3]) -> Self {
        let mut ray = Self {
            origin,
            hit: Intersection::new(),
            ..Default::default()
        };
        ray.set_direction(&dir);
        ray
    }

    pub fn set_direction(&mut self, dir: &[f32; 3]) {
        self.dir = normalize(&dir);
        self.r_d = [
            safercp(self.dir[0]),
            safercp(self.dir[1]),
            safercp(self.dir[2]),
        ];
    }
}

fn safercp(x: f32) -> f32 {
    if x > 1e-12 {
        return 1.0 / x;
    }
    if x < -1e-12 {
        return 1.0 / x;
    }
    f32::MAX
}
// Taken from tinybvh.
// TODO: Better to directly construct the C++ struct via its constructor.
fn length(a: &[f32; 3]) -> f32 {
    (a[0] * a[0] + a[1] * a[1] + a[2] * a[2]).sqrt()
}
fn normalize(a: &[f32; 3]) -> [f32; 3] {
    let l = length(a);
    let rl = if l == 0.0 { 0.0 } else { 1.0 / l };
    return [a[0] * rl, a[1] * rl, a[2] * rl];
}
