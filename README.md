# Show a potential bug in macros in rust lang

Files [macro.rs](https://github.com/AnickaBurova/macro-bug/blob/master/macro.rs) and [manual.rs](https://github.com/AnickaBurova/macro-bug/blob/master/manual.rs) are nearly identical, tho the macro.rs is failing with two errors.

The difference is the code which is failing to compile is generated using a macro.
``` bash
>>diff macro.rs manual.rs
178c178
<     #[cfg(feature = "manual")]
---
>     #[cfg(not(feature = "manual"))]

```

Those two files were generated using [cargo-expand](https://github.com/dtolnay/cargo-expand)

EDIT:
So this is not a bug, but hygiene feature of macros, the self should not leak in to the "code" block. I solved by making the code block lambda function and calling the function with all the necessary parameters. Compiler should be able to optimise the call out by inlining the code.

___```rust
    get_message!(CreateLight, Light: LightImpl, index: u8, name: String,
                 -> Lighting, create_light,
                 -> LightingImpl => {
                     |this, index, name, ctx|{
                        let addr = this
                            .lights
                            .entry(index)
                            .or_insert(LightImpl::new(ctx.address(), index));
                        (*addr).clone()
                     }
                 });
```

```rust
    impl Handler<$name> for $handler {
        type Result = Addr<$result_inner>;

        #[allow(unused_variables)]
        fn handle(&mut self, msg: $name, ctx: &mut Context<Self>) -> Self::Result {
            let $name { $($arg,)* } = msg;
            let fun: fn (&mut Self, $($arg_ty,)* &mut Context<Self>) -> Self::Result = $code;
            fun(self, $($arg,)* ctx)
        }
    }
```

