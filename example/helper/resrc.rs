//!
//! # Resources
//!

use crate::prelude::hal::{Peri, peripherals};
use assign_resources::assign_resources;

assign_resources! {
    /// for `Controller` task.
    main: ControllerSrc {}

    /// for `Blinky` task.
    blinky: BlinkySrc {
        iwdg_p: IWDG,
        led_pin: PC13,
    },
}
