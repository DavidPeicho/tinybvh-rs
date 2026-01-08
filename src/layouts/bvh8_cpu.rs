use crate::{cxx_ffi, ffi, mbvh, wald};

pub struct BVHData {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH8_CPU>,
}

impl BVHData {
    pub fn builder(mut self, original: &mbvh::BVHData) -> BVH {
        ffi::BVH8_CPU_setBVH(self.inner.pin_mut(), &original.inner);
        BVH {
            bvh: self,
            original,
        }
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
    original: &'a mbvh::BVHData,
}

impl<'a> BVH<'a> {
    pub fn new(original: &'a mbvh::BVHData) -> Self {
        let bvh = BVHData {
            inner: ffi::BVH8_CPU_new(),
        };
        let mut builder = bvh.builder(original);
        builder.convert();
        builder
    }

    pub fn convert(&mut self) {
        self.bvh
            .inner
            .pin_mut()
            .ConvertFrom(self.original.inner.as_ref().unwrap());
    }

    pub fn data(self) -> BVHData {
        self.bvh
    }
}
