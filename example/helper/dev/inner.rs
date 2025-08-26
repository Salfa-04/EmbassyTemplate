//!
//! # Device Inner Module
//!

use super::{AtomicBool, AtomicI8};
use super::{DevAddr, Device, Health, Order};

///
/// # Heartbeat Structure
///
pub(super) struct HeartBeat {
    online: AtomicBool,
    ttl: AtomicI8,
}

impl HeartBeat {
    ///
    ///  # Feed Heartbeat
    ///
    /// Set the device as online and reset its TTL (Time-To-Live) counter.
    ///
    pub(super) fn feed(&self, ttl: i8) {
        self.online.store(true, Order);
        self.ttl.store(ttl, Order);
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Set the device as offline and reset its TTL (Time-To-Live) counter to zero.
    ///
    pub(super) fn kill(&self) {
        self.online.store(false, Order);
        self.ttl.store(0, Order);
    }

    ///
    /// # Check Online Status
    ///
    /// Returns `true` if the device is online, `false` otherwise.
    ///
    pub(super) fn check(&self) -> bool {
        self.online.load(Order)
    }

    ///
    /// # Get TTL
    ///
    /// Returns the current TTL value.
    ///
    pub(super) fn ttl(&self) -> i8 {
        self.ttl.load(Order)
    }

    ///
    /// # Tick Heartbeat
    ///
    /// Decrement the TTL counter. If the counter reaches zero, mark the device as offline.
    ///
    pub(super) fn tick(&self) -> bool {
        let prev = self.ttl.fetch_sub(1, Order);
        if prev < 1 {
            self.ttl.store(0, Order);
            self.online.store(false, Order);
            return false; // Offline
        }

        true // Still Online
    }
}

impl Device {
    ///
    /// # Get Device ID
    ///
    /// Returns the CAN device ID associated with the device.
    ///
    pub const fn id(self) -> DevAddr {
        DevAddr::new(self as _)
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
