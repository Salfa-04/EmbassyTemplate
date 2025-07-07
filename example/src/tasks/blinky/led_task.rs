//!
//! # LED Task
//!

use crate::{hal, init_ticker};
use hal::{gpio, peripherals};

use gpio::{AnyPin, Level, Output as OP, Speed};

#[embassy_executor::task]
pub async fn task(p: (AnyPin,)) -> ! {
    let mut t = init_ticker!(150); // ms

    let mut led = OP::new(p.0, Level::Low, Speed::Low);

    loop {
        led.toggle();
        t.next().await;
    }
}
