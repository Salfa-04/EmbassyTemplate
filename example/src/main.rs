#![no_std]
#![no_main]

use utils::prelude::*;

mod controller;
mod tasks;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) -> ! {
    let (p,) = utils::res::sys_init();
    let r = {
        use utils::res::*;
        utils::split_resources!(p)
    };

    s.must_spawn(tasks::blinky::task(r.blinky));
    s.must_spawn(controller::main(r.main));

    utils::dev::Health::run().await
}
