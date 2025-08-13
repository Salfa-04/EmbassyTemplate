//!
//! # Utils
//!

#![no_std]
#![allow(unused_imports)]

use ::defmt_rtt as _;
use ::panic_probe as _;

pub mod bindings;
pub use init::sys_init;
pub use irq::IRQ;

mod init;
mod irq;
mod macros;

/// Preludes for easy imports.
pub mod prelude {
    /// # General Peripheral Pin Type
    /// Re-exporting commonly used types for convenience
    pub type GPIN<'t> = hal::Peri<'t, hal::gpio::AnyPin>;

    pub use ::cortex_m as ll; // Low Level
    pub use ::cortex_m_rt as rt; // Runtime
    pub use ::embassy_stm32 as hal; // HAL
    pub use ::embassy_sync as sync; // Sync
    pub use ::embassy_time::Timer as T; // Timer
}

#[::defmt::panic_handler]
fn soft_panic() -> ! {
    ::panic_probe::hard_fault()
}
