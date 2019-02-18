use actix::prelude::*;

use crate::lighting_impl::*;

pub struct Lighting {
    pub inner: Addr<LightingImpl>,
}

impl Lighting {
    pub fn new(inner: Addr<LightingImpl>) -> Self {
        Self { inner }
    }
}
