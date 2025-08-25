//!
//! # Device Address Wrapper
//!

use core::fmt::{Display, Formatter, Result};
use embedded_can::{ExtendedId, StandardId};

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

impl DevAddr {
    ///
    /// # As Standard ID
    ///
    /// Convert the device address to a StandardId.
    ///
    /// ## Panics if the address is not a valid StandardId.
    ///
    pub const fn as_standard_id(&self) -> StandardId {
        // Safety: Const-Fn can panic when Compiling.
        StandardId::new(self.as_raw() as _).unwrap()
    }

    ///
    /// # As Extended ID
    ///
    /// Convert the device address to an ExtendedId.
    ///
    /// ## Panics if the address is not a valid ExtendedId.
    ///
    pub const fn as_extended_id(&self) -> ExtendedId {
        // Safety: Const-Fn can panic when Compiling.
        ExtendedId::new(self.as_raw() as _).unwrap()
    }
}

impl Display for DevAddr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:#x?}", self.0)
    }
}
