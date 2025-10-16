//!
//! # Interrupts
//!

use utils::prelude::hal::{self, bind_interrupts, peripherals};

bind_interrupts! {
    pub struct IRQ {
        // USART3 => hal::usart::InterruptHandler<peripherals::USART3>;

        // CAN1_TX => hal::can::TxInterruptHandler<peripherals::CAN1>;
        // CAN1_RX0 => hal::can::Rx0InterruptHandler<peripherals::CAN1>;
        // CAN1_RX1 => hal::can::Rx1InterruptHandler<peripherals::CAN1>;
        // CAN1_SCE => hal::can::SceInterruptHandler<peripherals::CAN1>;

    }
}
