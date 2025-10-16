use crate::system::{ControllerSrc, SysMode};
use crate::tasks::health::Device;

#[embassy_executor::task]
pub async fn main(_p: ControllerSrc) {
    let mut t = utils::init_ticker!(1);

    SysMode::Boot.set();

    let device = Device::Placeholder;
    let _id = Device::Placeholder.id().as_raw();
    device.wait().await;

    SysMode::Normal.set();

    loop {
        if device.check() {
            device.kill();
        } else {
            device.feed();
        }

        t.next().await
    }
}
