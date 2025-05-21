use hal::{bind_interrupts, peripherals};
use {crate::hal, core::ffi::*};

bind_interrupts! {
    pub struct IntRqst {
        USART1 => hal::usart::InterruptHandler<peripherals::USART1>;
    }
}

#[link(name = "math", kind = "static")]
unsafe extern "C" {
    pub static mut c_buffer: [c_uchar; 16];

    pub fn c_add(a: c_int, b: c_int) -> c_int;
    pub fn c_sub(a: c_int, b: c_int) -> c_int;
    pub fn c_mul(a: c_int, b: c_int) -> c_int;
    pub fn c_div(a: c_int, b: c_int) -> c_int;
    pub fn c_mod(a: c_int, b: c_int) -> c_int;
}
