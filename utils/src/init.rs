//!
//! # System Initialization
//!

use crate::prelude::{hal, ll};
use hal::{Config, Peripherals, init, rcc, time::mhz};
use ll::{Peripherals as CorePeripherals, singleton};

///
/// # System Initialization Function
///
/// This function initializes the system peripherals and clocks.
///
pub fn sys_init() -> (CorePeripherals, Peripherals) {
    defmt::debug!("System Initialization...");

    if singleton!(:()=()).is_none() {
        panic!("Can Be Called Only Once!!!");
    }

    let Some(core) = CorePeripherals::take() else {
        panic!("Failed to take Core Peripherals!!!");
    };

    let peripherals = {
        let mut config = Config::default();
        let rcc = &mut config.rcc;

        init(config) // SysClock = xMHz
    };

    (core, peripherals)
}
