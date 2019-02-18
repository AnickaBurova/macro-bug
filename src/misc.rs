macro_rules! get_message {
    ($name: ident, $result: ty: $result_inner: ty, $($arg: ident: $arg_ty: ty,)* -> $main: ty, $fun: ident, -> $handler: ty => $code: block) => {
        pub struct $name {
            $(pub $arg: $arg_ty,)*
        }

        impl Message for $name {
            type Result = Addr<$result_inner>;
        }

        impl $name {
            pub fn new($($arg: $arg_ty,)*) -> Self {
                Self {
                    $($arg,)*
                }
            }
        }

        impl $main {
            pub fn $fun(&self, $($arg: $arg_ty,)*) -> impl Future<Item = $result, Error = Error> {
                self.inner
                    .send($name::new($($arg,)*))
                    .map(|inner| Light::new(inner))
                    .map_err(Error::from)
            }
        }

        #[cfg(not(feature = "manual"))]
        impl Handler<$name> for $handler {
            type Result = Addr<$result_inner>;

            #[allow(unused_variables)]
            fn handle(&mut self, msg: $name, ctx: &mut Context<Self>) -> Self::Result {
                let $name { $($arg,)* } = msg;
                $code
            }
        }
    }
}
