/// Class wrapper around [`syntect::highlighting::Color`]
// PyO3 imports
use pyo3::prelude::*;

// Syntect imports
use syntect::highlighting::Color;

/// RGBA color descriptor
#[pyclass(name = "Color")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorWrap {
    /// Handle to original color
    pub color: Color,
}

#[pymethods]
impl ColorWrap {
    /// Red component.
    #[getter]
    fn r(&self) -> u8 {
        self.color.r
    }

    /// Green component.
    #[getter]
    fn g(&self) -> u8 {
        self.color.g
    }

    /// Blue component.
    #[getter]
    fn b(&self) -> u8 {
        self.color.b
    }

    /// Alpha component.
    #[getter]
    fn a(&self) -> u8 {
        self.color.a
    }
}
