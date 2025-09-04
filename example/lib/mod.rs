//!
//! # Utils
//!

#![no_std]
#![allow(unused_imports)]

use ::defmt_rtt as _;
use ::panic_probe as _;
use core::sync::atomic;

use atomic::Ordering::Relaxed as Order;
use atomic::{AtomicBool, AtomicI8};
pub use status::SysMode;

pub mod dev;
mod macros;
mod status;

/// Preludes for easy imports.
pub mod prelude {
    pub use ::cortex_m as ll; // Low Level
    pub use ::cortex_m_rt as rt; // Runtime
    pub use ::embassy_futures as ef; // Futures
    pub use ::embassy_stm32 as hal; // HAL
    pub use ::embassy_sync as sync; // Sync
    pub use ::embassy_time::Timer as T; // Timer
}

/// Resource Management
pub mod res {
    mod init;
    mod irq;
    mod resrc;

    pub use init::sys_init;
    pub use irq::IRQ;
    pub use resrc::*;
}

#[::defmt::panic_handler]
fn soft_panic() -> ! {
    ::panic_probe::hard_fault()
}
