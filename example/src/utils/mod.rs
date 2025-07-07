//!
//! # Utils
//!

#![allow(unused_imports)]
use ::defmt_rtt as _;
use ::panic_probe as _;

pub mod bindings;
pub use init::sys_init;
pub use irq::IRQ;

mod init;
mod macros;
mod irq;

pub mod prelude {
    pub use ::bitfield_struct::bitfield; // Bitfield
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
