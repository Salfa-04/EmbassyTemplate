//!
//! # System Module
//!

#![allow(dead_code)]
#![allow(unused_imports)]

use atomic::Ordering::Relaxed as Order;
use core::sync::atomic::{self, AtomicI8};

mod interrupts;
mod resources;
mod status;

pub use interrupts::IRQ;
pub use resources::*;
pub use status::SysMode;
