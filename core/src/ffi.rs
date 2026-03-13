//! Python FFI bindings using PyO3
#![allow(non_local_definitions)]

use crate::{Route, Router, RouterConfig};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

/// Python wrapper for Router
#[pyclass(name = "PyRouter")]
pub struct PyRouter {
    router: Router,
}

#[pymethods]
impl PyRouter {
    /// Create new router
    #[new]
    #[pyo3(signature = (dimension=384, threshold=0.5))]
    fn new(dimension: usize, threshold: f32) -> PyResult<Self> {
        if dimension == 0 {
            return Err(PyRuntimeError::new_err("Dimension must be positive"));
        }

        if !(0.0..=1.0).contains(&threshold) {
            return Err(PyRuntimeError::new_err("Threshold must be between 0 and 1"));
        }

        let config = RouterConfig {
            dimension,
            default_threshold: threshold,
            top_k: 5,
            enable_calibration: true,
        };

        match config.validate() {
            Ok(_) => Ok(Self {
                router: Router::new(config),
            }),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    /// Add route to router
    fn add_route(&mut self, route: PyRoute) -> PyResult<()> {
        if route.id.is_empty() {
            return Err(PyRuntimeError::new_err("Route ID cannot be empty"));
        }

        if route.examples.is_empty() && route.description.is_empty() {
            return Err(PyRuntimeError::new_err(format!(
                "Route '{}' must have examples or description",
                route.id
            )));
        }

        let rust_route = Route {
            id: route.id,
            description: route.description,
            examples: route.examples,
            keywords: route.keywords,
            patterns: vec![],
            metadata: HashMap::new(),
            threshold: None,
            tags: vec![],
        };

        self.router
            .add_route(rust_route)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// Build routing index
    fn build_index(&mut self, embeddings: Vec<Vec<f32>>) -> PyResult<()> {
        if embeddings.is_empty() {
            return Err(PyRuntimeError::new_err("Embeddings cannot be empty"));
        }

        self.router
            .build_index(embeddings)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))
    }

    /// Route query
    fn route(&mut self, text: String, embedding: Vec<f32>) -> PyResult<Py<PyDict>> {
        if text.trim().is_empty() {
            return Err(PyRuntimeError::new_err("Query text cannot be empty"));
        }

        if embedding.is_empty() {
            return Err(PyRuntimeError::new_err("Embedding cannot be empty"));
        }

        let result = self
            .router
            .route(&text, &embedding)
            .map_err(|e| PyRuntimeError::new_err(e.to_string()))?;

        Python::with_gil(|py| {
            let dict = PyDict::new(py);
            dict.set_item("route_id", result.route_id)?;
            dict.set_item("confidence", result.scores.confidence)?;

            let scores = PyDict::new(py);
            scores.set_item("semantic", result.scores.semantic)?;
            scores.set_item("keyword", result.scores.keyword)?;
            scores.set_item("pattern", result.scores.pattern)?;
            scores.set_item("total", result.scores.total)?;
            scores.set_item("confidence", result.scores.confidence)?;
            dict.set_item("scores", scores)?;

            dict.set_item("latency_ms", result.latency_ms)?;

            Ok(dict.into())
        })
    }

    /// Get number of routes
    fn route_count(&self) -> usize {
        self.router.route_count()
    }

    /// Check if index is built
    fn is_index_built(&self) -> bool {
        self.router.is_index_built()
    }
}

/// Python wrapper for Route
#[pyclass(name = "PyRoute")]
#[derive(Clone)]
pub struct PyRoute {
    /// Route identifier
    #[pyo3(get, set)]
    pub id: String,
    /// Route description
    #[pyo3(get, set)]
    pub description: String,
    /// Example queries
    #[pyo3(get, set)]
    pub examples: Vec<String>,
    /// Route keywords
    #[pyo3(get, set)]
    pub keywords: Vec<String>,
}

#[pymethods]
impl PyRoute {
    /// Create new route
    #[new]
    fn new(id: String) -> PyResult<Self> {
        if id.trim().is_empty() {
            return Err(PyRuntimeError::new_err("Route ID cannot be empty"));
        }

        Ok(Self {
            id,
            description: String::new(),
            examples: Vec::new(),
            keywords: Vec::new(),
        })
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "PyRoute(id='{}', keywords={})",
            self.id,
            self.keywords.len()
        )
    }
}
