//!
//! # System Initialization
//!

use crate::prelude::hal::{Config, init, rcc, time::mhz};

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
        config.enable_debug_during_sleep = true;

        let rcc = &mut config.rcc;

        rcc.hsi = false; // HSI = 16MHz
        rcc.hse = Some(rcc::Hse {
            freq: mhz(12),
            mode: rcc::HseMode::Oscillator,
        });

        rcc.pll_src = rcc::PllSource::HSE; // HSE = 12MHz
        rcc.pll = Some(rcc::Pll {
            prediv: rcc::PllPreDiv::DIV6,   //   2MHz
            mul: rcc::PllMul::MUL168,       // 336MHz
            divp: Some(rcc::PllPDiv::DIV2), // 168MHz
            divq: Some(rcc::PllQDiv::DIV7), //  48MHz
            divr: None,                     // Not used
        });

        rcc.plli2s = None; // Not used

        rcc.sys = rcc::Sysclk::PLL1_P; // 168MHz
        rcc.ahb_pre = rcc::AHBPrescaler::DIV1; // 168MHz
        rcc.apb1_pre = rcc::APBPrescaler::DIV4; // 42MHz
        rcc.apb2_pre = rcc::APBPrescaler::DIV2; // 84MHz

        rcc.ls = rcc::LsConfig::default_lsi(); // LSI = 32KHz
        rcc.mux.clk48sel = rcc::mux::Clk48sel::PLL1_Q; // 48MHz
        rcc.mux.rtcsel = rcc::mux::Rtcsel::DISABLE; // Disabled
        rcc.mux.sdiosel = rcc::mux::Sdiosel::CLK48; // 48MHz

        init(config) // SysClock = 168MHz
    };

    (peripherals,)
}
