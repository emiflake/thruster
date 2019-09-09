use crate::algebra::prelude::*;

use crate::bxdf::BxDF;

pub struct BSDF<'a> {
    pub bxdfs: &'a [&'a dyn BxDF],
    pub geometric_normal: Normal,
    pub shading_normal: Normal,
}
