//!
//! # System Initialization
//!

use crate::prelude::hal::{init, rcc, time::mhz, Config};

///
/// # System Initialization Function
///
/// This function initializes the system peripherals and clocks.
///
pub fn sys_init() -> (embassy_stm32::Peripherals,) {
    defmt::debug!("System Initialization...");

    if cortex_m::singleton!(:()=()).is_none() {
        panic!("Can Be Called Only Once!!!");
    }

    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        init(config) // SysClock = xMHz
    };

    (peripherals,)
}
