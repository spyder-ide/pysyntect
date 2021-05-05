/// Class wrapper around [`syntect::highlighting::Style`]

// PyO3 imports
use pyo3::prelude::*;

// Syntect imports
use syntect::highlighting::Style;

// Local imports
use crate::base::ColorWrap;


/// Style color description used to highlight a token.
#[pyclass(name="Style")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleWrap {
    /// Foreground color.
    pub foreground: ColorWrap,
    /// Background color.
    pub background: ColorWrap,
    /// Font style
    pub font_style: u8,
    /// Handle to original style
    pub style: Style,
}

#[pymethods]
impl StyleWrap {
    /// Foreground color.
    #[getter]
    fn foreground(&self) -> ColorWrap {
        self.foreground
    }

    /// Background color.
    #[getter]
    fn background(&self) -> ColorWrap {
        self.background
    }

    /// Font style.
    #[getter]
    fn font_style(&self) -> u8 {
        self.font_style
    }
}
