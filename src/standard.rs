use embedded_hal::digital::InputPin;

use crate::Direction;
use crate::RotaryEncoder;

/// StandardMode
/// This mode is best used when polled at ~900Hz.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StandardMode {
    /// The pin state
    pin_state: [u8; 2],
    direction: Direction,
    count: i128,
}

// For debouncing of pins, use 0x0f (b00001111) and 0x0c (b00001100) etc.
const PIN_MASK: u8 = 0x03;
const PIN_EDGE: u8 = 0x02;

impl<DT, CLK> RotaryEncoder<StandardMode, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Updates the `RotaryEncoder`, updating the `direction` property
    pub fn update(&mut self) -> Direction {
        self.mode.update(
            self.pin_dt.is_high().unwrap_or_default(),
            self.pin_clk.is_high().unwrap_or_default(),
        )
    }
    /// Updates the count on each iteration. Call when using self-count feature after `update()`.
    pub fn update_count(&mut self) {
        self.mode.update_count()
    }

    /// Call after update and between iterations
    pub fn count(&self) -> i128 {
        self.mode.count
    }

    /// Query encoder direction
    pub fn direction(&self) -> Direction {
        self.mode.direction
    }
}

impl StandardMode {
    /// Initialises the StandardMode
    pub fn new() -> Self {
        Self {
            pin_state: [0xFF, 2],
            direction: Direction::None,
            count: 0,
        }
    }

    /// Update to determine the direction
    pub fn update(&mut self, dt_value: bool, clk_value: bool) -> Direction {
        self.pin_state[0] = (self.pin_state[0] << 1) | dt_value as u8;
        self.pin_state[1] = (self.pin_state[1] << 1) | clk_value as u8;

        let a = self.pin_state[0] & PIN_MASK;
        let b = self.pin_state[1] & PIN_MASK;

        let mut direction: Direction = Direction::None;

        let a_is_pin_edge = a == PIN_EDGE;
        let b_is_pin_edge = b == PIN_EDGE;

        if a_is_pin_edge {
            self.count += 1;
        }
        if b_is_pin_edge {
            self.count -= 1;
        }
        if a_is_pin_edge && b == 0x00 {
            direction = Direction::Anticlockwise;
        } else if b_is_pin_edge && a == 0x00 {
            direction = Direction::Clockwise;
        }

        self.direction = direction;

        direction
    }

    /// Updates the count on each iteration. Call when using self-count feature after `update()`.
    pub fn update_count(&mut self) {
        match self.direction {
            Direction::None => {}
            Direction::Clockwise => {
                self.count += 1;
            }
            Direction::Anticlockwise => {
                self.count -= 1;
            }
        }
    }

    /// Call after update and between iterations
    pub fn count(&self) -> i128 {
        self.count
    }

    /// Query encoder direction
    pub fn direction(&self) -> Direction {
        self.direction
    }
}

impl<LOGIC, DT, CLK> RotaryEncoder<LOGIC, DT, CLK>
where
    DT: InputPin,
    CLK: InputPin,
{
    /// Configure `RotaryEncoder` to use the standard API
    pub fn into_standard_mode(self) -> RotaryEncoder<StandardMode, DT, CLK> {
        RotaryEncoder {
            pin_dt: self.pin_dt,
            pin_clk: self.pin_clk,
            mode: StandardMode::new(),
        }
    }
}

impl Default for StandardMode {
    fn default() -> Self {
        Self::new()
    }
}
