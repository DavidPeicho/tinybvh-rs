use std::fmt;
use crate::BVHLayoutType;

pub struct MissingLayout(BVHLayoutType);

impl MissingLayout {
    pub fn new(layout: BVHLayoutType) -> Self {
        Self { 0: layout }
    }
}

impl fmt::Debug for MissingLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Missing layout {:?}. Convert the bvh first using convert(layout)", self.0)
    }
}

pub enum BVHCompactError {
    UnsupportedLayout(BVHLayoutType),
    MissingLayout(BVHLayoutType),
}

impl fmt::Debug for BVHCompactError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BVHCompactError::UnsupportedLayout(l) => {
                write!(f, "Unsupported layout type for operation: {:?}", l)
            },
            Self::MissingLayout(l) => {
                MissingLayout::new(*l).fmt(f)
            }
        }
    }
}

impl From<MissingLayout> for BVHCompactError {
    fn from(value: MissingLayout) -> Self {
        BVHCompactError::MissingLayout(value.0)
    }
}
