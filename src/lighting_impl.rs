use crate::light_impl::LightImpl;
use crate::lighting::Lighting;
use actix::prelude::*;
use failure::Error;
use futures::future::Future;
use std::collections::HashMap;

#[derive(Clone)]
pub struct LightingImpl {
    prefix: String,
    pack: u8,
    lights: HashMap<u8, Addr<LightImpl>>,
}

impl LightingImpl {
    pub fn new(pack: u8) -> Addr<LightingImpl> {
        LightingImpl::create(move |_| {
            let prefix = format!("_{:00}L", pack);
            LightingImpl { prefix, pack, lights: HashMap::new() }
        })
    }
    fn get_prefix(&self) -> &str {
        &self.prefix
    }
}

impl Actor for LightingImpl {
    type Context = Context<Self>;
}

use super::light::*;
get_message!(CreateLight, Light: LightImpl, index: u8, name: String,
             -> Lighting, create_light,
             -> LightingImpl => {
                let addr = self
                    .lights
                    .entry(index)
                    .or_insert(LightImpl::new(ctx.address(), index));
                (*addr).clone()
             });

#[cfg(feature = "manual")]
impl Handler<CreateLight> for LightingImpl {
    type Result = Addr<LightImpl>;
    #[allow(unused_variables)]
    fn handle(&mut self, msg: CreateLight, ctx: &mut Context<Self>) -> Self::Result {
        let CreateLight { index, name } = msg;
        {
            let addr = self.lights.entry(index).or_insert(LightImpl::new(ctx.address(), index));
            (*addr).clone()
        }
    }
}
