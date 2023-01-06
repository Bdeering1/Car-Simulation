use std::fmt::Display;

pub struct Transmission {
    gear_ratios: Vec<f64>,
    diff_ratio: f64,
    pub gear: u8,
    pub max_gear: u8,
}

impl Transmission {
    pub fn new(diff_ratio: f64, gear_ratios: Vec<f64>) -> Self {
        Self {
            max_gear: (gear_ratios.len() - 1) as u8,
            gear_ratios,
            diff_ratio,
            gear: 1,
        }
    }

    pub fn get_ratio(&self) -> f64 {
        (*self.gear_ratios.get(self.gear as usize).expect("Error: invalid gear")) * self.diff_ratio
    }
}

impl Display for Transmission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("gear: {}", self.gear - 1).as_str())?;
        Ok(())
    }
}