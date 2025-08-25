//!
//! # Device Manager Module
//!

use atomic::Ordering::Relaxed as Order;
use atomic::{AtomicBool, AtomicI8};
use core::sync::atomic;
use inner::HeartBeat;

pub use devid::DevAddr;
pub use heart::Health;
pub use status::SysMode;

mod devid;
mod heart;
mod inner;
mod status;

///
/// # Health Watch List
///
const WATCH_LIST: &[Device] = &[
    Device::ChaMotA,
    Device::ChaMotB,
    Device::ChaMotC,
    Device::ChaMotD,
];

///
/// # Device Enumeration
///
#[repr(usize)]
#[derive(defmt::Format, Debug, PartialEq)]
pub enum Device {
    ChaCtrl = 0x200,
    ChaMotA = 0x201,
    ChaMotB = 0x202,
    ChaMotC = 0x203,
    ChaMotD = 0x204,
}
