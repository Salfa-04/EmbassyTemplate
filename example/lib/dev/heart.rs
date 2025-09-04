//!
//! # Heartbeat Monitor
//!

use super::{Device, WATCH_LIST};
use crate::{AtomicBool, AtomicI8, Order};

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
    /// Get Health Check Interval
    pub const fn interval() -> u8 {
        Self::HEALTH_MS
    }

    /// Get Number of Monitored Devices
    const fn amount() -> usize {
        assert!(
            ADDR.len() == STATE.len(),
            "Addr and State Length Mismatched!"
        );

        ADDR.len()
    }
}

impl Health {
    ///
    /// # Build Device-Heartbeat Pairs
    ///
    pub fn build() -> [(&'static Device, &'static HeartBeat); Self::amount()] {
        let mut counter = 0usize..;
        [(); _].map(|_| {
            // Safety: Counter is always in bound
            let idx = counter.next().unwrap();
            // Safety: idx is always in bound
            (&ADDR[idx], &STATE[idx])
        })
    }

    ///
    /// # Tick Heartbeat
    ///
    /// Decrement the TTL counter.
    ///
    /// - `true` if the device is still online.
    /// - `false` if the device has gone offline.
    ///
    pub fn tick(heart: &HeartBeat) -> bool {
        heart.tick()
    }
}

impl Device {
    ///
    /// # Feed Heartbeat
    ///
    /// Feed the heartbeat for this device.
    ///
    #[inline(always)]
    pub fn feed(&self) {
        Health::feed(self)
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Kill the heartbeat for this device.
    ///
    #[inline(always)]
    pub fn kill(&self) {
        Health::kill(self)
    }

    ///
    /// # Check Heartbeat
    ///
    /// Check if the heartbeat for this device is alive.
    ///
    #[inline(always)]
    pub fn check(&self) -> bool {
        Health::check(self)
    }

    ///
    /// # Wait for Device to be Online
    ///
    /// Returns a future that resolves when the device is online.
    ///
    #[inline(always)]
    pub fn wait(&self) -> impl core::future::Future<Output = ()> {
        Health::wait_for(self)
    }
}

impl Health {
    ///
    /// # Feed Heartbeat
    ///
    /// Feed the heartbeat for the specified device address.
    ///
    fn feed(addr: &Device) {
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
    fn kill(addr: &Device) {
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
    fn check(addr: &Device) -> bool {
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
    fn wait_for(addr: &Device) -> impl core::future::Future<Output = ()> {
        if let Some(idx) = ADDR.iter().position(|x| x == addr) {
            async move {
                // Safety: idx is guaranteed to be in bound
                while !STATE[idx].check() {
                    crate::prelude::T::after_millis(Self::WAIT_INTERVAL_MS as _).await
                }
            }
        } else {
            panic!("Invalid Address: {:?}", addr);
        }
    }
}

///
/// # Heartbeat Structure
///
pub struct HeartBeat {
    online: AtomicBool,
    ttl: AtomicI8,
}

impl HeartBeat {
    ///
    ///  # Feed Heartbeat
    ///
    /// Set the device as online and reset its TTL (Time-To-Live) counter.
    ///
    fn feed(&self, ttl: i8) {
        self.online.store(true, Order);
        self.ttl.store(ttl, Order);
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Set the device as offline and reset its TTL (Time-To-Live) counter to zero.
    ///
    fn kill(&self) {
        self.online.store(false, Order);
        self.ttl.store(0, Order);
    }

    ///
    /// # Check Online Status
    ///
    /// Returns `true` if the device is online, `false` otherwise.
    ///
    fn check(&self) -> bool {
        self.online.load(Order)
    }

    ///
    /// # Get TTL
    ///
    /// Returns the current TTL value.
    ///
    fn ttl(&self) -> i8 {
        self.ttl.load(Order)
    }

    ///
    /// # Tick Heartbeat
    ///
    /// Decrement the TTL counter.
    ///
    /// If the counter reaches zero, mark the device as offline.
    ///
    fn tick(&self) -> bool {
        let prev = self.ttl.fetch_sub(1, Order);
        if prev < 1 {
            self.ttl.store(0, Order);
            self.online.store(false, Order);
            return false; // Offline
        }

        true // Still Online
    }
}

impl defmt::Format for Health {
    fn format(&self, fmt: defmt::Formatter) {
        if STATE.is_empty() {
            defmt::write!(fmt, "No Devices Monitored");
            return;
        }

        for (idx, x) in STATE.iter().enumerate() {
            defmt::write!(fmt, "\nDevice {}: ", ADDR[idx]);
            if x.check() {
                defmt::write!(fmt, "Online (TTL={})", x.ttl());
            } else {
                defmt::write!(fmt, "Offline (TTL<0)");
            }
        }
    }
}
