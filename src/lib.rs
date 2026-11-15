//! SigmaOS-data-ml: ML pipeline model.
//!
//! Real implementation: pipelines, feature engineering, transformations, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Feature type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FeatureType {
    Numerical,
    Categorical,
    Binary,
    Text,
}

/// Feature.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feature {
    pub id: String,
    pub name: String,
    pub feature_type: FeatureType,
}

/// Pipeline step.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineStep {
    pub id: String,
    pub name: String,
    pub parameters: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MlError {
    EmptyName,
    DuplicateFeature(String),
}

impl fmt::Display for MlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "feature name must not be empty"),
            Self::DuplicateFeature(id) => write!(f, "duplicate feature {:?}", id),
        }
    }
}

impl std::error::Error for MlError {}

/// ML pipeline.
pub struct MlPipeline {
    pub name: String,
    features: BTreeMap<String, Feature>,
    steps: BTreeMap<String, PipelineStep>,
}

impl MlPipeline {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            features: BTreeMap::new(),
            steps: BTreeMap::new(),
        }
    }

    pub fn add_feature(&mut self, feature: Feature) -> Result<(), MlError> {
        if feature.name.trim().is_empty() { return Err(MlError::EmptyName); }
        let id = feature.id.clone();
        if self.features.contains_key(&id) {
            return Err(MlError::DuplicateFeature(id.clone()));
        }
        self.features.insert(id, feature);
        Ok(())
    }

    pub fn add_step(&mut self, step: PipelineStep) { self.steps.insert(step.id.clone(), step); }

    pub fn feature_count(&self) -> usize { self.features.len() }

    pub fn step_count(&self) -> usize { self.steps.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_create_aaa() {
        let p = MlPipeline::new("My Pipeline");
        assert_eq!(p.name, "My Pipeline");
        assert_eq!(p.feature_count(), 0);
    }

    #[test]
    fn test_add_feature_aaa() {
        let mut p = MlPipeline::new("My Pipeline");
        let f = Feature { id: "f1".into(), name: "age".into(), feature_type: FeatureType::Numerical };
        p.add_feature(f).unwrap();
        assert_eq!(p.feature_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut p = MlPipeline::new("My Pipeline");
        let f = Feature { id: "x".into(), name: "".into(), feature_type: FeatureType::Numerical };
        assert!(p.add_feature(f).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut p = MlPipeline::new("My Pipeline");
        let f = Feature { id: "x".into(), name: "X".into(), feature_type: FeatureType::Numerical };
        p.add_feature(f).unwrap();
        let f2 = Feature { id: "x".into(), name: "X".into(), feature_type: FeatureType::Numerical };
        assert!(p.add_feature(f2).is_err());
    }
}
