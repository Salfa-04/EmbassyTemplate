//!
//! # Health Task
//!

use super::Health;
use utils::init_ticker;

/// Device Enumeration
#[repr(usize)]
#[derive(defmt::Format, Debug, PartialEq)]
pub enum Device {
    Placeholder = 0x0000,
}

#[embassy_executor::task]
pub async fn task() -> ! {
    utils::T::after_millis(1000).await;
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

/// Health Watch List
pub(super) const WATCH_LIST: &[Device] = &[
    // Device::Placeholder,
];

/// Settings for Heartbeat Monitoring
impl Health {
    /// Health Check Interval in ms
    pub(super) const HEALTH_MS: u8 = 100;
    /// Device Expiration Time in ms
    pub(super) const EXPIRE_MS: u16 = 500;
    /// Wait interval in ms when waiting for a device to be online
    pub(super) const WAIT_INTERVAL_MS: u8 = 10;
}
