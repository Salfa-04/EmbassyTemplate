#![no_std]
#![no_main]

use utils::prelude::*;
use utils::res::*;

mod controller;
mod tasks;

#[embassy_executor::main]
async fn entry(s: embassy_executor::Spawner) -> ! {
    let (p,) = sys_init();
    let p = utils::split_resources!(p);

    s.must_spawn(tasks::blinky::task(p.blinky));
    s.must_spawn(controller::main(p.main));

    utils::dev::Health::run().await
}
