use crate::system::{Device, SysMode};

#[embassy_executor::task]
pub async fn main() {
    let mut t = utils::init_ticker!(1);

    SysMode::Boot.set();

    let device = Device::Placeholder;
    Device::wait(&Device::Placeholder, &mut t).await;

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
