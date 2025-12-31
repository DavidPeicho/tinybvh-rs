use crate::{cxx_ffi, ffi, mbvh, wald};

pub struct BVH {
    pub(crate) inner: cxx::UniquePtr<ffi::BVH8_CPU>,
}

impl BVH {
    pub fn builder(mut self, original: &mbvh::BVH) -> Builder {
        ffi::BVH8_CPU_setBVH(self.inner.pin_mut(), &original.inner);
        Builder {
            bvh: self,
            original,
        }
    }
}

// TODO: Only with AVX2
impl crate::Intersector for BVH {
    fn intersect(&self, ray: &mut crate::Ray) -> u32 {
        self.inner.Intersect(ray) as u32
    }
}

pub struct Builder<'a> {
    bvh: BVH,
    original: &'a mbvh::BVH,
}

impl<'a> Builder<'a> {
    pub fn new(original: &'a mbvh::BVH) -> Self {
        let bvh = BVH {
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

    pub fn bvh(self) -> BVH {
        self.bvh
    }
}
