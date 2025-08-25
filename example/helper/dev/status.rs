//!
//! # System Status
//!

use super::{AtomicI8, Order};

static STATUS: AtomicI8 = AtomicI8::new(SysMode::Boot as _);

#[must_use]
#[non_exhaustive]
#[repr(i8)]
pub enum SysMode {
    Error = -1,
    Boot = 0,
    NoForce = 1,
    Normal = 2,
}

impl SysMode {
    const ERROR: i8 = Self::Error as _;
    const BOOT: i8 = Self::Boot as _;
    const NOFORCE: i8 = Self::NoForce as _;
    const NORMAL: i8 = Self::Normal as _;
}

impl SysMode {
    pub fn set(self) {
        STATUS.store(self as _, Order);
    }

    pub fn get() -> SysMode {
        match STATUS.load(Order) {
            Self::ERROR => Self::Error,
            Self::BOOT => Self::Boot,
            Self::NOFORCE => Self::NoForce,
            Self::NORMAL => Self::Normal,

            _ => panic!("Invalid System Mode!"),
        }
    }
}
