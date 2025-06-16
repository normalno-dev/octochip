use super::quircks::Quircks;
pub struct Config {
    pub quircks: Quircks,
    pub cpu_frequency: u16,
    pub timer_frequency: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            quircks: Quircks::default(),
            cpu_frequency: 500,
            timer_frequency: 60,
        }
    }
}
