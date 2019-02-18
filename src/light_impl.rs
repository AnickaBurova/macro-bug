use crate::lighting_impl::LightingImpl;
use actix::prelude::*;

#[derive(Clone)]
pub struct LightImpl {
    prefix: String,
    parent: Addr<LightingImpl>,
    index: u8,
}

impl LightImpl {
    pub fn new(root: Addr<LightingImpl>, index: u8) -> Addr<Self> {
        LightImpl::create(move |_| {
            let prefix = format!("{:00}", index);
            LightImpl { prefix, parent: root, index }
        })
    }
}

impl Actor for LightImpl {
    type Context = Context<Self>;
}
