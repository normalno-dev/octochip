use std::time::Duration;

use crate::{display::Display, keyboard::Keyboard};

pub enum ExecutionMode {
    Running,
    Paused,
    Step,
}

pub trait Platform {
    type Error: From<crate::error::Error>;

    fn get_keys(&self) -> Keyboard;

    fn draw_display(&mut self, display: &Display) -> Result<(), Self::Error>;
    fn play_sound(&mut self, enabled: bool) -> Result<(), Self::Error>;

    fn get_time(&self) -> Duration;

    fn get_execution_mode(&self) -> ExecutionMode;
}
