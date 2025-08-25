//!
//! # Heartbeat Monitor
//!

use super::{AtomicBool, AtomicI8};
use super::{Device, Order, WATCH_LIST};
use crate::{init_ticker, prelude::T};

const ADDR: &[Device] = const { WATCH_LIST };

static STATE: [HeartBeat; ADDR.len()] = unsafe { core::mem::zeroed() };

pub struct Health;

impl Health {
    const HEALTH_MS: u8 = 100;
    const EXPIRE_MS: u16 = 500;
}

impl Health {
    pub fn feed(addr: &Device) {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            let ttl = const { (Self::EXPIRE_MS / Self::HEALTH_MS as u16) as i8 };
            // Safety: idx is guaranteed to be in bound
            STATE[idx].feed(ttl);
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }

    pub fn kill(addr: &Device) {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            // Safety: idx is guaranteed to be in bound
            STATE[idx].kill();
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }

    pub fn check(addr: &Device) -> bool {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            // Safety: idx is guaranteed to be in bound
            STATE[idx].check()
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }
}

impl Health {
    pub async fn run() -> ! {
        let mut t = init_ticker!(Self::HEALTH_MS as u64);
        (T::after_millis(1000).await, t.reset());

        loop {
            for (idx, x) in STATE.iter().enumerate() {
                if !x.tick() {
                    defmt::warn!("Device Offline: {}", ADDR[idx]);
                }
            }

            t.next().await
        }
    }
}

struct HeartBeat {
    online: AtomicBool,
    ttl: AtomicI8,
}

impl HeartBeat {
    fn feed(&self, ttl: i8) {
        self.online.store(true, Order);
        self.ttl.store(ttl, Order);
    }

    fn kill(&self) {
        self.online.store(false, Order);
        self.ttl.store(0, Order);
    }

    fn check(&self) -> bool {
        self.online.load(Order)
    }

    fn tick(&self) -> bool {
        let prev = self.ttl.fetch_sub(1, Order);
        if prev <= 1 {
            self.ttl.store(0, Order);
            self.online.store(false, Order);
            return false; // Offline
        }

        true // Still Online
    }
}
