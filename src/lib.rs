use pyo3::create_exception;
use pyo3::exceptions;
/// PyO3 imports
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Syntect imports
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Class wrappers around syntect structs
#[pyclass(name=Color)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorWrap {
    /// Red component
    pub r: u8,
    /// Green component
    pub g: u8,
    /// Blue component
    pub b: u8,
    /// Alpha component
    pub a: u8,
}

#[pymethods]
impl ColorWrap {
    #[getter]
    fn r(&self) -> PyResult<u8> {
        Ok(self.r)
    }

    #[getter]
    fn g(&self) -> PyResult<u8> {
        Ok(self.g)
    }

    #[getter]
    fn b(&self) -> PyResult<u8> {
        Ok(self.b)
    }

    #[getter]
    fn a(&self) -> PyResult<u8> {
        Ok(self.a)
    }
}

#[pyclass(name=Style)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleWrap {
    /// Foreground color.
    pub foreground: ColorWrap,
    /// Background color.
    pub background: ColorWrap,
    /// Font style
    pub font_style: u8,
}

#[pymethods]
impl StyleWrap {
    #[getter]
    fn foreground(&self) -> PyResult<ColorWrap> {
        Ok(self.foreground)
    }

    #[getter]
    fn background(&self) -> PyResult<ColorWrap> {
        Ok(self.background)
    }

    #[getter]
    fn font_style(&self) -> PyResult<u8> {
        Ok(self.font_style)
    }
}

#[pyclass(name=FontStyleConst)]
pub struct FontStyleWrap {}

#[pymethods]
impl FontStyleWrap {
    /// Bold font style
    pub const BOLD: i8 = 1;
    /// Underline font style
    pub const UNDERLINE: i8 = 2;
    /// Italic font style
    pub const ITALIC: i8 = 4;

    #[staticmethod]
    fn bold() -> PyResult<i8> {
        Ok(FontStyleWrap::BOLD)
    }

    #[staticmethod]
    fn underline() -> PyResult<i8> {
        Ok(FontStyleWrap::UNDERLINE)
    }

    #[staticmethod]
    fn italic() -> PyResult<i8> {
        Ok(FontStyleWrap::ITALIC)
    }
}

#[pyclass(name=SyntaxSet)]
#[derive(Debug, Clone)]
pub struct SyntaxSetHandle {
    pub syntax_set: SyntaxSet,
}

#[pymethods]
impl SyntaxSetHandle {
    #[getter]
    fn languages(&self) -> PyResult<Vec<&str>> {
        let mut languages = Vec::<&str>::new();
        for syntax in self.syntax_set.syntaxes() {
            languages.push(&syntax.name);
        }
        Ok(languages)
    }

    fn __len__(&self) -> PyResult<usize> {
        Ok(self.syntax_set.syntaxes().len())
    }
}

// Exception classes
create_exception!(pysyntect, LoadingError, exceptions::Exception);

// Module methods
#[pyfunction]
/// Test function for syntect execution
fn highlight(
    text: &'static str,
    language: &str,
    syntax_set: SyntaxSetHandle,
) -> PyResult<Vec<(StyleWrap, &'static str)>> {
    // let ps = SyntaxSet::load_defaults_newlines();
    let ps = syntax_set.syntax_set;
    let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(language).unwrap();
    // for key in ts.themes.keys() {
    //     println!("{}", key);
    // }
    let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (dark)"]);
    // let s = "def tefa(a, b, *args, **kwargs):\n    return a + b";
    let mut python_output = Vec::<(StyleWrap, &str)>::new();
    for line in LinesWithEndings::from(text) {
        let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
        for &(ref style, text) in ranges.iter() {
            let py_foreground = ColorWrap {
                r: style.foreground.r,
                g: style.foreground.g,
                b: style.foreground.b,
                a: style.foreground.a,
            };

            let py_background = ColorWrap {
                r: style.background.r,
                g: style.background.g,
                b: style.background.b,
                a: style.background.a,
            };

            let py_style = StyleWrap {
                foreground: py_foreground,
                background: py_background,
                font_style: style.font_style.bits(),
            };

            python_output.push((py_style, text));
        }
        let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
        println!("{}", escaped);
    }
    Ok(python_output)
}

#[pyfunction]
fn load_syntax_folder(folder: &'static str) -> PyResult<SyntaxSetHandle> {
    let syn_set = SyntaxSet::load_from_folder(folder);
    match syn_set {
        Ok(result) => {
            let syntaxes = result.syntaxes();
            let syn_handle = SyntaxSetHandle { syntax_set: result };
            Ok(syn_handle)
        }
        Err(err) => Err(LoadingError::py_err(err.to_string())),
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn syntect(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(highlight))?;
    m.add_wrapped(wrap_pyfunction!(load_syntax_folder))?;
    m.add_class::<StyleWrap>()?;
    m.add_class::<ColorWrap>()?;
    m.add_class::<FontStyleWrap>()?;
    m.add_class::<SyntaxSetHandle>()?;
    Ok(())
}
