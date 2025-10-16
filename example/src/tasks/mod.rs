//!
//! # Tasks
//!

pub mod blinky {
    mod task;

    pub use task::task;
}

pub mod health {
    mod devid;
    mod heart;
    mod task;

    use heart::Health;
    use task::WATCH_LIST;

    pub use task::Device;
    pub use task::task;

    #[allow(unused_imports)]
    pub use devid::DevAddr;
}
