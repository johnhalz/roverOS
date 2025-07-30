use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MobilityConfig {
    pub version: String,
    pub body: BodyConfig,
    pub motors: HashMap<String, MotorConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BodyConfig {
    pub max_speed: f64,         // [m/s]
    pub max_acceleration: f64,  // [m/s^2]
    pub min_turn_radius: f64,   // [m]
    pub max_standup_speed: f64, // [m/s]
    pub max_roll_angle: f64,    // [degrees]
    pub max_pitch_angle: f64,   // [degrees]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MotorConfig {
    pub name: String,
}

impl MobilityConfig {
    /// Deserialize from TOML string
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Get a specific motor configuration by name
    pub fn get_motor(&self, motor_name: &str) -> Option<&MotorConfig> {
        self.motors.get(motor_name)
    }

    /// Get all motor names
    pub fn motor_names(&self) -> Vec<&String> {
        self.motors.keys().collect()
    }
}
