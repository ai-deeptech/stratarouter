//! Per-route score normalisation using a fixed piecewise-linear curve.
//!
//! The current implementation is a **piecewise-linear interpolator** over
//! a pre-defined calibration table. Full online isotonic-regression fitting
//! (learn from routing feedback) is planned for a future release.

use std::collections::HashMap;

/// Piecewise-linear score normaliser.
///
/// Maps a raw fused score in `[0, 1]` to a calibrated probability estimate
/// also in `[0, 1]`. Monotonicity is guaranteed by the calibration table.
pub struct ScoreNormalizer {
    thresholds: Vec<f32>,
    calibrated_values: Vec<f32>,
}

impl ScoreNormalizer {
    /// Create a normaliser with the default calibration table.
    pub fn new() -> Self {
        Self {
            thresholds: vec![0.0, 0.2, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0],
            calibrated_values: vec![0.01, 0.15, 0.35, 0.48, 0.62, 0.76, 0.88, 0.96, 0.99],
        }
    }

    /// Normalise `raw_score` to a `(calibrated, uncertainty)` pair.
    ///
    /// `uncertainty` is a fixed estimate (`0.05`) until online calibration
    /// is implemented.
    pub fn calibrate(&self, raw_score: f32) -> (f32, f32) {
        let raw_score = raw_score.clamp(0.0, 1.0);

        let idx = self
            .thresholds
            .binary_search_by(|t| t.partial_cmp(&raw_score).unwrap())
            .unwrap_or_else(|i| i.saturating_sub(1));

        if idx + 1 >= self.calibrated_values.len() {
            let last = self.calibrated_values.len() - 1;
            return (self.calibrated_values[last], 0.05);
        }

        let t1 = self.thresholds[idx];
        let t2 = self.thresholds[idx + 1];
        let v1 = self.calibrated_values[idx];
        let v2 = self.calibrated_values[idx + 1];

        let weight = if t2 - t1 > 0.0 {
            (raw_score - t1) / (t2 - t1)
        } else {
            0.0
        };

        let calibrated = (v1 + weight * (v2 - v1)).clamp(0.0, 1.0);
        (calibrated, 0.05)
    }
}

impl Default for ScoreNormalizer {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages per-route score normalisation.
pub struct CalibrationManager {
    normalizers: HashMap<String, ScoreNormalizer>,
}

impl CalibrationManager {
    /// Create a new manager with no per-route overrides.
    pub fn new() -> Self {
        Self {
            normalizers: HashMap::new(),
        }
    }

    /// Return a calibrated `(score, uncertainty)` pair for `route_id`.
    ///
    /// A per-route normaliser is lazily initialised with default settings on
    /// first access.
    pub fn calibrate_for_route(&mut self, route_id: &str, raw_score: f32) -> (f32, f32) {
        let normalizer = self
            .normalizers
            .entry(route_id.to_string())
            .or_default();
        normalizer.calibrate(raw_score)
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
    fn test_calibration_in_range() {
        let n = ScoreNormalizer::new();
        let (c, u) = n.calibrate(0.75);
        assert!(c > 0.7 && c < 0.9);
        assert!(u > 0.0);
    }

    #[test]
    fn test_calibration_bounds() {
        let n = ScoreNormalizer::new();
        let (low, _) = n.calibrate(0.0);
        let (high, _) = n.calibrate(1.0);
        assert!(low < high);
        assert!(low >= 0.0 && high <= 1.0);
    }

    #[test]
    fn test_calibration_monotone() {
        let n = ScoreNormalizer::new();
        let (s1, _) = n.calibrate(0.3);
        let (s2, _) = n.calibrate(0.7);
        assert!(s2 >= s1, "Normaliser must be monotonically non-decreasing");
    }

    #[test]
    fn test_manager_default_per_route() {
        let mut m = CalibrationManager::new();
        let (s1, _) = m.calibrate_for_route("route_a", 0.5);
        let (s2, _) = m.calibrate_for_route("route_b", 0.5);
        assert!((s1 - s2).abs() < 0.01, "Default normaliser should be identical for all routes");
    }
}
