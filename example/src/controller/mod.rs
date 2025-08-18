use crate::{ControllerSrc};

#[embassy_executor::task]
pub async fn main(_p: ControllerSrc) {
    let mut t = utils::init_ticker!(20);

    loop {
        t.next().await
    }
}
