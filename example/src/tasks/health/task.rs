//!
//! # Health Task
//!

use utils::{dev::Health, init_ticker};

#[embassy_executor::task]
pub async fn task() -> ! {
    crate::T::after_millis(1000).await;
    let mut t = init_ticker!(Health::interval() as u64);

    let device: _ = Health::build();

    loop {
        for (addr, heart) in &device {
            if !Health::tick(heart) {
                defmt::warn!("Device Offline: {:?}", addr);
            }
        }

        // defmt::debug!("Health: {:?}", Health);

        t.next().await
    }
}
