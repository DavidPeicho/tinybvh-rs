use crate::Ray;

pub trait Intersector {
    fn intersect(&self, ray: &mut Ray) -> u32;
}
