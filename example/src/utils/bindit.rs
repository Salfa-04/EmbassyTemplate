use crate::hal::{self, bind_interrupts, peripherals};

bind_interrupts! {
    pub struct IntRqst {
        UART4 => hal::usart::InterruptHandler<peripherals::UART4>;
    }
}
