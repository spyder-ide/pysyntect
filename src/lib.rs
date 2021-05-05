// ----------------------------------------------------------------------------
// Copyright (c) Spyder Project Contributors
//
// Licensed under the terms of the MIT License
// (see LICENSE.txt for details)
// ----------------------------------------------------------------------------

/// Rust bindings to syntect library in Python

// Local modules
mod base;
mod exceptions;
mod highlighting;
mod escape;
mod loading;

// PyO3 imports
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// Local imports
use self::base::FontStyleWrap;
use self::base::color::ColorWrap;
use self::base::style::StyleWrap;
use self::base::syntax::{SyntaxHandle, SyntaxSetHandle};
use self::base::theme::{ThemeHandle, ThemeSetHandle};
use self::exceptions::{LoadingError, SyntaxNotFoundError};
use self::highlighting::__pyo3_get_function_highlight;
use self::escape::{__pyo3_get_function_escape_to_console, __pyo3_get_function_escape_to_latex};
use self::loading::{__pyo3_get_function_load_syntax_folder, __pyo3_get_function_load_theme_folder};

// Package version
const VERSION: &str = env!("CARGO_PKG_VERSION");


/// This module is a python module implemented in Rust.
#[pymodule]
fn syntect(py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add("LoadingError", py.get_type::<LoadingError>())?;
    m.add("SyntaxNotFoundError", py.get_type::<SyntaxNotFoundError>())?;
    m.add_function(wrap_pyfunction!(highlight, m)?)?;
    m.add_function(wrap_pyfunction!(load_syntax_folder, m)?)?;
    m.add_function(wrap_pyfunction!(load_theme_folder, m)?)?;
    m.add_function(wrap_pyfunction!(escape_to_console, m)?)?;
    m.add_function(wrap_pyfunction!(escape_to_latex, m)?)?;
    m.add_class::<StyleWrap>()?;
    m.add_class::<ColorWrap>()?;
    m.add_class::<FontStyleWrap>()?;
    m.add_class::<SyntaxHandle>()?;
    m.add_class::<SyntaxSetHandle>()?;
    m.add_class::<ThemeHandle>()?;
    m.add_class::<ThemeSetHandle>()?;
    Ok(())
}
