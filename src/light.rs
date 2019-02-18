use crate::light_impl::*;
use actix::prelude::*;

pub struct Light {
    inner: Addr<LightImpl>,
}

impl Light {
    pub fn new(inner: Addr<LightImpl>) -> Self {
        Self { inner }
    }
}
