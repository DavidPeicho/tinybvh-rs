use crate::{ffi, layouts::impl_bvh_deref, mbvh};

pub struct BVHData {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH8_CPU>,
}

impl BVHData {
    pub fn bvh<'a>(mut self, original: &'a mbvh::BVH) -> BVH<'a> {
        ffi::BVH8_CPU_setBVH(self.inner.pin_mut(), &original.inner);
        BVH {
            bvh: self,
            original,
        }
    }

    pub fn convert<'a>(mut self, original: &'a mbvh::BVH) -> BVH<'a> {
        BVH {
            bvh: self,
            original,
        }
        .convert(original)
    }
}

#[cfg(target_feature = "avx2")]
impl crate::Intersector for BVHData {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}

pub struct BVH<'a> {
    bvh: BVHData,
    original: &'a mbvh::BVH<'a>,
}

impl<'a> BVH<'a> {
    pub fn new(original: &'a mbvh::BVH) -> Self {
        BVHData {
            inner: ffi::BVH8_CPU_new(),
        }
        .convert(original)
    }

    pub fn convert<'b>(mut self, original: &'b mbvh::BVH) -> BVH<'b> {
        let mut bvh = self.bvh;
        bvh.inner
            .pin_mut()
            .ConvertFrom(self.original.inner.as_ref().unwrap());
        BVH { bvh, original }
    }

    pub fn data(self) -> BVHData {
        self.bvh
    }
}

impl_bvh_deref!(BVH<'a>, BVHData);
