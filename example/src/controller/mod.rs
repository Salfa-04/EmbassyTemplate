use crate::{ControllerSrc, init_ticker};

#[embassy_executor::task]
pub async fn main(_p: ControllerSrc) {
    let mut t = init_ticker!(20);

    loop {
        t.next().await
    }
}
