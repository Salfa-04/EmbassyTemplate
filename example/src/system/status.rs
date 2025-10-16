//!
//! # System Status
//!

use super::{AtomicI8, Order};

static STATUS: AtomicI8 = AtomicI8::new(SysMode::Boot as _);

///
/// # System Mode Enumeration
///
/// ## Get Current Mode
/// ```rust
/// let mode: SysMode = SysMode::get();
/// ```
///
/// ## Set Current Mode
/// ```rust
/// SysMode::Normal.set();
/// SysMode::set(SysMode::Normal);
/// ```
///
#[repr(i8)]
#[non_exhaustive]
#[derive(defmt::Format, Debug)]
pub enum SysMode {
    Error = -1,
    Boot = 0,
    Normal = 1,
}

impl SysMode {
    const ERROR: i8 = Self::Error as _;
    const BOOT: i8 = Self::Boot as _;
    const NORMAL: i8 = Self::Normal as _;
}

impl SysMode {
    ///
    /// # Get System Mode
    ///
    /// Retrieve the current system mode.
    ///
    pub fn get() -> SysMode {
        match STATUS.load(Order) {
            Self::ERROR => Self::Error,
            Self::BOOT => Self::Boot,
            Self::NORMAL => Self::Normal,

            x => panic!("Invalid System Mode: {x}"),
        }
    }

    ///
    /// # Set System Mode
    ///
    /// Set the current system mode to the specified value.
    ///
    pub fn set(self) {
        STATUS.store(self as _, Order);
    }
}
