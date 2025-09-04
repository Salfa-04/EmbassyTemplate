//!
//! # Device ID Helper
//!

///
/// # Device Address Wrapper
///
#[repr(transparent)]
pub struct DevAddr(usize);

impl DevAddr {
    ///
    /// # Create New Device Address
    ///
    /// Create a new device address from a raw usize value.
    ///
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    ///
    /// # Get Raw Address
    ///
    /// Get the raw usize address value.
    ///
    pub const fn as_raw(&self) -> usize {
        self.0
    }
}

impl super::Device {
    ///
    /// # Get Device ID
    ///
    /// Returns the CAN device ID associated with the device.
    ///
    pub const fn id(self) -> DevAddr {
        DevAddr::new(self as _)
    }
}

impl defmt::Format for DevAddr {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:#x}", self.0)
    }
}
