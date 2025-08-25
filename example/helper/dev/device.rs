//!
//! # Device Address Wrapper
//!

use core::fmt::{Display, Formatter, Result};
use embedded_can::{ExtendedId, StandardId};

#[repr(transparent)]
pub struct DevAddr(usize);

impl DevAddr {
    pub const fn new(addr: usize) -> Self {
        Self(addr)
    }

    pub const fn as_raw(&self) -> usize {
        self.0
    }
}

impl DevAddr {
    pub const fn as_standard_id(&self) -> StandardId {
        // Safety: Const-Fn can panic when Compiling.
        StandardId::new(self.as_raw() as _).unwrap()
    }

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
