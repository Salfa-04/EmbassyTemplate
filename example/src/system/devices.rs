//!
//! # System Devices
//!

use super::private::*;

type Pair = (&'static Device, &'static HeartBeat);

static PAIRS: [Pair; WATCH_LIST.len()] = {
    const N: usize = WATCH_LIST.len();
    static STATE: [HeartBeat; N] = unsafe { core::mem::zeroed() };
    let mut pairs: _ = match N {
        0 => [],
        _ => [(&WATCH_LIST[0], &STATE[0]); _],
    };
    let mut i = 0;
    while i < N {
        pairs[i] = (&WATCH_LIST[i], &STATE[i]);
        i += 1;
    }
    pairs
};

impl Device {
    ///
    /// # Get Heartbeat
    ///
    /// Get the heartbeat associated with this device.
    ///
    fn heartbeat(&self) -> Option<&'static HeartBeat> {
        PAIRS.iter().find(|&&(addr, _)| addr == self).map(|x| x.1)
    }

    ///
    /// # Maximum TTL
    ///
    /// Calculate the maximum Time-To-Live (TTL) value.
    ///
    const fn max_ttl() -> i8 {
        (Self::EXPIRE_MS / Self::HEALTH_MS as u16) as i8
    }

    ///
    /// # Get Health Check Interval
    ///
    /// Returns the health check interval in milliseconds.
    ///
    pub const fn interval() -> u64 {
        Self::HEALTH_MS as _
    }
}

impl Device {
    ///
    /// # Feed Heartbeat
    ///
    /// Feed the heartbeat for this device.
    ///
    pub fn feed(&self) {
        match self.heartbeat() {
            Some(x) => {
                x.feed(Self::max_ttl());
            }
            None => panic!("Invalid Address: {:?}", self),
        }
    }

    ///
    /// # Kill Heartbeat
    ///
    /// Kill the heartbeat for this device.
    ///
    pub fn kill(&self) {
        match self.heartbeat() {
            Some(x) => x.kill(),
            None => panic!("Invalid Address: {:?}", self),
        }
    }

    ///
    /// # Check Heartbeat
    ///
    /// Check if the heartbeat for this device is alive.
    ///
    pub fn check(&self) -> bool {
        match self.heartbeat() {
            Some(x) => x.check(),
            None => panic!("Invalid Address: {:?}", self),
        }
    }

    ///
    /// # Wait for Device to be Online
    ///
    /// Returns a future that resolves when the device is online.
    ///
    pub fn wait(&self, t: &mut Ticker) -> impl core::future::Future<Output = ()> {
        let heart = match self.heartbeat() {
            Some(x) => x,
            None => panic!("Invalid Address: {:?}", self),
        };

        async {
            while !heart.check() {
                t.next().await
            }
        }
    }

    ///
    /// # Tick Heartbeat
    ///
    /// Decrement the TTL counter.
    ///
    /// - `true` if the device is still online.
    /// - `false` if the device has gone offline.
    ///
    pub fn tick(&self) -> bool {
        match self.heartbeat() {
            Some(x) => x.tick(),
            None => panic!("Invalid Address: {:?}", self),
        }
    }
}

impl defmt::Format for Device {
    fn format(&self, fmt: defmt::Formatter) {
        let device = self.heartbeat();

        if let Some(heart) = device {
            if heart.check() {
                defmt::write!(fmt, "{:?} (Online, TTL={})", self, heart.ttl());
            } else {
                defmt::write!(fmt, "{:?} (Offline)", self);
            }
        } else {
            defmt::write!(fmt, "{:?} (No Heartbeat)", self);
        }
    }
}
