#[macro_use]
mod misc;

mod light;
mod light_impl;
mod lighting;
mod lighting_impl;
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
                    println!("We got a light");
                    System::current().stop();
                    Ok(())
                })
                .map_err(|err| eprintln!("Err: {}", err)),
        )
    });
}
