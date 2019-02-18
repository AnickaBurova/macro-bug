#![feature(prelude_import)]
#![no_std]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std as std;
#[macro_use]
mod misc {}
mod light {
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
}
mod light_impl {
    use crate::lighting_impl::LightingImpl;
    use actix::prelude::*;
    pub struct LightImpl {
        prefix: String,
        parent: Addr<LightingImpl>,
        index: u8,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LightImpl {
        #[inline]
        fn clone(&self) -> LightImpl {
            match *self {
                LightImpl {
                    prefix: ref __self_0_0,
                    parent: ref __self_0_1,
                    index: ref __self_0_2,
                } => LightImpl {
                    prefix: ::std::clone::Clone::clone(&(*__self_0_0)),
                    parent: ::std::clone::Clone::clone(&(*__self_0_1)),
                    index: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl LightImpl {
        pub fn new(root: Addr<LightingImpl>, index: u8) -> Addr<Self> {
            LightImpl::create(move |_| {
                let prefix = ::alloc::fmt::format(::std::fmt::Arguments::new_v1_formatted(
                    &[""],
                    &match (&index,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                    &[::std::fmt::rt::v1::Argument {
                        position: ::std::fmt::rt::v1::Position::At(0usize),
                        format: ::std::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::std::fmt::rt::v1::Count::Implied,
                            width: ::std::fmt::rt::v1::Count::Is(0usize),
                        },
                    }],
                ));
                LightImpl {
                    prefix,
                    parent: root,
                    index,
                }
            })
        }
    }
    impl Actor for LightImpl {
        type Context = Context<Self>;
    }
}
mod lighting {
    use crate::lighting_impl::*;
    use actix::prelude::*;
    pub struct Lighting {
        pub inner: Addr<LightingImpl>,
    }
    impl Lighting {
        pub fn new(inner: Addr<LightingImpl>) -> Self {
            Self { inner }
        }
    }
}
mod lighting_impl {
    use crate::light_impl::LightImpl;
    use crate::lighting::Lighting;
    use actix::prelude::*;
    use failure::Error;
    use futures::future::Future;
    use std::collections::HashMap;
    pub struct LightingImpl {
        prefix: String,
        pack: u8,
        lights: HashMap<u8, Addr<LightImpl>>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::std::clone::Clone for LightingImpl {
        #[inline]
        fn clone(&self) -> LightingImpl {
            match *self {
                LightingImpl {
                    prefix: ref __self_0_0,
                    pack: ref __self_0_1,
                    lights: ref __self_0_2,
                } => LightingImpl {
                    prefix: ::std::clone::Clone::clone(&(*__self_0_0)),
                    pack: ::std::clone::Clone::clone(&(*__self_0_1)),
                    lights: ::std::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    impl LightingImpl {
        pub fn new(pack: u8) -> Addr<LightingImpl> {
            LightingImpl::create(move |_| {
                let prefix = ::alloc::fmt::format(::std::fmt::Arguments::new_v1_formatted(
                    &["_", "L"],
                    &match (&pack,) {
                        (arg0,) => [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)],
                    },
                    &[::std::fmt::rt::v1::Argument {
                        position: ::std::fmt::rt::v1::Position::At(0usize),
                        format: ::std::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::std::fmt::rt::v1::Alignment::Unknown,
                            flags: 8u32,
                            precision: ::std::fmt::rt::v1::Count::Implied,
                            width: ::std::fmt::rt::v1::Count::Is(0usize),
                        },
                    }],
                ));
                LightingImpl {
                    prefix,
                    pack,
                    lights: HashMap::new(),
                }
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
    pub struct CreateLight {
        pub index: u8,
        pub name: String,
    }
    impl Message for CreateLight {
        type Result = Addr<LightImpl>;
    }
    impl CreateLight {
        pub fn new(index: u8, name: String) -> Self {
            Self { index, name }
        }
    }
    impl Lighting {
        pub fn create_light(
            &self,
            index: u8,
            name: String,
        ) -> impl Future<Item = Light, Error = Error> {
            self.inner
                .send(CreateLight::new(index, name))
                .map(|inner| Light::new(inner))
                .map_err(Error::from)
        }
    }
    #[cfg(not(feature = "manual"))]
    impl Handler<CreateLight> for LightingImpl {
        type Result = Addr<LightImpl>;
        #[allow(unused_variables)]
        fn handle(&mut self, msg: CreateLight, ctx: &mut Context<Self>) -> Self::Result {
            let CreateLight { index, name } = msg;
            {
                let addr = self
                    .lights
                    .entry(index)
                    .or_insert(LightImpl::new(ctx.address(), index));
                (*addr).clone()
            }
        }
    }
}
use actix::prelude::*;
use futures::future::Future;
use lighting::*;
use lighting_impl::*;
fn main() {
    System::run(|| {
        let li = LightingImpl::new(3);
        let l = Lighting::new(li);
        Arbiter::spawn(
            l.create_light(12, "Light".into())
                .and_then(|light| {
                    {
                        ::std::io::_print(::std::fmt::Arguments::new_v1(
                            &["We got a light\n"],
                            &match () {
                                () => [],
                            },
                        ));
                    };
                    System::current().stop();
                    Ok(())
                })
                .map_err(|err| {
                    ::std::io::_eprint(::std::fmt::Arguments::new_v1(
                        &["Err: ", "\n"],
                        &match (&err,) {
                            (arg0,) => {
                                [::std::fmt::ArgumentV1::new(arg0, ::std::fmt::Display::fmt)]
                            }
                        },
                    ));
                }),
        )
    });
}
