//!
//! # Heartbeat Monitor Module
//!

use super::{Device, HeartBeat, WATCH_LIST};
use crate::{init_ticker, prelude::T};

///
/// # Device Address List
///
/// This is a constant array of device addresses that are being monitored.
///
const ADDR: &[Device] = const { WATCH_LIST };

///
/// # Static Heartbeat State Array
///
/// Safety: This is safe because it is only accessed in a controlled manner.
///
static STATE: [HeartBeat; ADDR.len()] = unsafe { core::mem::zeroed() };

///
/// # Health Monitor
///
pub struct Health;

impl Health {
    /// Health Check Interval in ms
    const HEALTH_MS: u8 = 100;
    /// Device Expiration Time in ms
    const EXPIRE_MS: u16 = 500;
    /// Wait interval in ms when waiting for a device to be online
    const WAIT_INTERVAL_MS: u8 = 10;
}

impl Health {
    ///
    /// # Feed Heartbeat
    ///
    /// Feed the heartbeat for the specified device address.
    ///
    pub fn feed(addr: &Device) {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            let ttl = const { (Self::EXPIRE_MS / Self::HEALTH_MS as u16) as i8 };
            // Safety: idx is guaranteed to be in bound
            STATE[idx].feed(ttl);
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Kill the heartbeat for the specified device address.
    ///
    pub fn kill(addr: &Device) {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            // Safety: idx is guaranteed to be in bound
            STATE[idx].kill();
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }

    ///
    /// # Check Heartbeat
    ///
    /// Check if the specified device is online.
    ///
    pub fn check(addr: &Device) -> bool {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            // Safety: idx is guaranteed to be in bound
            STATE[idx].check()
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }

    ///
    /// # Wait for Device to be Online
    ///
    /// This function returns a future that resolves when the specified device is online.
    ///
    pub fn wait_for(addr: &Device) -> impl core::future::Future<Output = ()> {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            async move {
                // Safety: idx is guaranteed to be in bound
                while !STATE[idx].check() {
                    T::after_millis(Self::WAIT_INTERVAL_MS as _).await
                }
            }
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }
}

impl Health {
    ///
    /// # Run Health Monitor
    ///
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

impl defmt::Format for Health {
    fn format(&self, fmt: defmt::Formatter) {
        for (idx, x) in STATE.iter().enumerate() {
            defmt::write!(fmt, "Device {}: ", ADDR[idx]);
            if x.check() {
                defmt::write!(fmt, "Online (TTL={})\n", x.ttl());
            } else {
                defmt::write!(fmt, "Offline (TTL<0)\n");
            }
        }
    }
}
