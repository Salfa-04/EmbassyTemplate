//!
//! # Device Manager
//!

pub use devid::DevAddr;
pub use heart::Health;

mod devid;
mod heart;

///
/// # Device Enumeration
///
#[repr(usize)]
#[derive(defmt::Format, Debug, PartialEq)]
pub enum Device {
    Placeholder = 0x0000,
}

///
/// # Health Watch List
///
const WATCH_LIST: &[Device] = &[
    // Device::Placeholder,
];

///
/// # Settings for Heartbeat Monitoring
///
impl Health {
    /// Health Check Interval in ms
    const HEALTH_MS: u8 = 100;
    /// Device Expiration Time in ms
    const EXPIRE_MS: u16 = 500;
    /// Wait interval in ms when waiting for a device to be online
    const WAIT_INTERVAL_MS: u8 = 10;
}
