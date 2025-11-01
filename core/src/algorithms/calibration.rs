//! Per-route calibration using isotonic regression

use std::collections::HashMap;

/// Isotonic calibrator for score normalization
pub struct IsotonicCalibrator {
    thresholds: Vec<f32>,
    calibrated_values: Vec<f32>,
}

impl IsotonicCalibrator {
    /// Create calibrator with default curve
    pub fn new() -> Self {
        Self {
            thresholds: vec![0.0, 0.2, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
            calibrated_values: vec![0.01, 0.15, 0.35, 0.48, 0.62, 0.76, 0.88, 0.96, 0.99],
        }
    }
    
    /// Calibrate raw score
    pub fn calibrate(&self, raw_score: f32) -> (f32, f32) {
        let raw_score = raw_score.clamp(0.0, 1.0);
        
        // Find position in thresholds
        let idx = self.thresholds
            .binary_search_by(|t| t.partial_cmp(&raw_score).unwrap())
            .unwrap_or_else(|i| i.saturating_sub(1));
        
        // Handle edge case at end
        if idx + 1 >= self.calibrated_values.len() {
            let last = self.calibrated_values.len() - 1;
            return (self.calibrated_values[last], 0.05);
        }
        
        // Linear interpolation between points
        let t1 = self.thresholds[idx];
        let t2 = self.thresholds[idx + 1];
        let v1 = self.calibrated_values[idx];
        let v2 = self.calibrated_values[idx + 1];
        
        let weight = if t2 - t1 > 0.0 {
            (raw_score - t1) / (t2 - t1)
        } else {
            0.0
        };
        
        let calibrated = v1 + weight * (v2 - v1);
        
        (calibrated.clamp(0.0, 1.0), 0.05) // Fixed uncertainty
    }
}

impl Default for IsotonicCalibrator {
    fn default() -> Self {
        Self::new()
    }
}

/// Calibration manager for all routes
pub struct CalibrationManager {
    calibrators: HashMap<String, IsotonicCalibrator>,
}

impl CalibrationManager {
    /// Create new manager
    pub fn new() -> Self {
        Self {
            calibrators: HashMap::new(),
        }
    }
    
    /// Calibrate score for specific route
    pub fn calibrate_for_route(&mut self, route_id: &str, raw_score: f32) -> (f32, f32) {
        let calibrator = self.calibrators
            .entry(route_id.to_string())
            .or_default();
        
        calibrator.calibrate(raw_score)
    }
}

impl Default for CalibrationManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_calibration() {
        let calibrator = IsotonicCalibrator::new();
        let (calibrated, uncertainty) = calibrator.calibrate(0.75);
        assert!(calibrated > 0.7 && calibrated < 0.9);
        assert!(uncertainty > 0.0);
    }
    
    #[test]
    fn test_calibration_bounds() {
        let calibrator = IsotonicCalibrator::new();
        
        let (low, _) = calibrator.calibrate(0.0);
        let (high, _) = calibrator.calibrate(1.0);
        
        assert!(low < high);
        assert!(low >= 0.0 && high <= 1.0);
    }
    
    #[test]
    fn test_calibration_monotonic() {
        let calibrator = IsotonicCalibrator::new();
        
        let (score1, _) = calibrator.calibrate(0.3);
        let (score2, _) = calibrator.calibrate(0.7);
        
        assert!(score2 >= score1);
    }
    
    #[test]
    fn test_calibration_manager() {
        let mut manager = CalibrationManager::new();
        
        let (score1, _) = manager.calibrate_for_route("route1", 0.5);
        let (score2, _) = manager.calibrate_for_route("route2", 0.5);
        
        // Should be same for default calibrators
        assert!((score1 - score2).abs() < 0.01);
    }
}
