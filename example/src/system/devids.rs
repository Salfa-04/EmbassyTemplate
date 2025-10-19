//!
//! # System Device IDs
//!

use super::private::*;

///
/// #  Device Address Wrapper
///
#[repr(transparent)]
pub struct DevAddr(usize);

impl Device {
    ///
    /// # Get Device ID
    ///
    pub const fn id(self) -> DevAddr {
        DevAddr::new(self as _)
    }
}

impl DevAddr {
    ///
    /// # Create New Device Address
    ///
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    ///
    /// # Get Raw Address
    ///
    pub const fn as_raw(&self) -> usize {
        self.0
    }
}

// /**
//  *                 Impl for DevAddr                   *
//  */
// use embedded_can::{ExtendedId, StandardId};

// impl DevAddr {
//     pub const fn as_standard_id(&self) -> StandardId {
//         // Safety: Const-Fn can panic when Compiling.
//         StandardId::new(self.as_raw() as _).unwrap()
//     }

//     pub const fn as_extended_id(&self) -> ExtendedId {
//         // Safety: Const-Fn can panic when Compiling.
//         ExtendedId::new(self.as_raw() as _).unwrap()
//     }
// }

impl defmt::Format for DevAddr {
    fn format(&self, fmt: defmt::Formatter) {
        defmt::write!(fmt, "{:#x}", self.0)
    }
}
