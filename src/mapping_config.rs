use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingConfig {
    pub version: String,
    pub devices: HashMap<String, MappingDeviceConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MappingDeviceConfig {
    pub name: String,
    pub lens: String,
    pub vision: VisionConfig,
    pub object_detection: ObjectDetectionConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VisionConfig {
    pub capture_resolution: (u32, u32), // (width, height)
    pub capture_framerate: f64,         // [fps]
    pub limit_perception_distance: f64, // [m]
    pub limit_perception_angle: f64,    // [degrees]
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectDetectionConfig {
    pub detection_enabled: bool,
    pub tracking_enabled: bool,
    pub max_detection_range: f64,       // [m]
    pub objects: Vec<String>,           // List of object types to detect
    pub confidence_threshold: f64,      // [0.0 - 1.0]
    pub detection_outputs: Vec<String>, // List of output channels for detections
    pub skeleton_tracking_enabled: bool,
    pub skeleton_tracking_keypoint_count: usize, // Number of keypoints to track
}

impl MappingConfig {
    /// Deserialize from TOML string
    pub fn from_toml(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// Get a specific motor configuration by name
    pub fn get_device(&self, device_name: &str) -> Option<&MappingDeviceConfig> {
        self.devices.get(device_name)
    }

    /// Get all motor names
    pub fn device_names(&self) -> Vec<&String> {
        self.devices.keys().collect()
    }
}
