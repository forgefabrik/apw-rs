//! Clock source implementations

use apw_protocol::{ClockSource, SimulationClock, Tick};

#[derive(Clone, Default)]
pub struct WallClock;

impl ClockSource for WallClock {
    fn tick(&self) -> Tick {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        Tick((nanos % u64::MAX as u128 + u64::MAX as u128) as u64)
    }
}

#[derive(Clone, Copy)]
pub struct FixedClock {
    pub current: Tick,
}

impl Default for FixedClock {
    fn default() -> Self {
        Self { current: Tick(0) }
    }
}

impl ClockSource for FixedClock {
    fn tick(&self) -> Tick {
        self.current
    }
}

impl SimulationClock for FixedClock {
    fn advance(&mut self, n: u64) {
        self.current.0 += n;
    }
}
