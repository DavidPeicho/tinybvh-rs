#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Intersection {
    t: f32,
    u: f32,
    v: f32,
    prim: u32,
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
        Self {
            origin,
            dir,
            ..Default::default()
        }
    }
}
