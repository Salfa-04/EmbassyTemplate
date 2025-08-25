//!
//! # Device Manager Module
//!

use atomic::Ordering::Relaxed as Order;
use atomic::{AtomicBool, AtomicI8};
use core::sync::atomic;

pub use device::DevAddr;
pub use heart::Health;
pub use status::SysMode;

mod device;
mod heart;
mod status;

/// Health Watch List
const WATCH_LIST: &[Device] = &[
    // Device::None,
];

#[repr(usize)]
#[derive(defmt::Format, Debug, PartialEq)]
pub enum Device {
    None = 0x0000,
}

impl Device {
    pub fn feed(&self) {
        Health::feed(&self)
    }

    pub fn kill(&self) {
        Health::kill(&self)
    }

    pub fn check(&self) -> bool {
        Health::check(&self)
    }
}

impl Device {
    pub const fn id(self) -> DevAddr {
        DevAddr::new(self as _)
    }
}
