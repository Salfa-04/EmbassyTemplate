//!
//! # Health Task
//!

use crate::system::Device;
use crate::system::WATCH_LIST;
use utils::init_ticker;

#[embassy_executor::task]
pub async fn task() -> ! {
    let mut t = init_ticker!(Device::interval(), ms);

    loop {
        for device in WATCH_LIST {
            if !device.tick() {
                defmt::warn!("Device Offline: {:?}", device);
            }
        }

        // defmt::debug!("Health: {:?}", WATCH_LIST);

        t.next().await
    }
}
