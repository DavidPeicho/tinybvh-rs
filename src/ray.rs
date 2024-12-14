use core::f32;

use crate::ffi;

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
            t: crate::INFINITE,
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
        ffi::ray_new(&origin, &dir)
    }
}
