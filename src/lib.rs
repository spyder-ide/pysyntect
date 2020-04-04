/// PyO3 imports
use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::class::PyMappingProtocol;

/// Syntect imports
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Package version
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
    /// Handle to original style
    style: Style,
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

#[pyclass(name=Syntax)]
pub struct SyntaxHandle {
    pub syntax: SyntaxReference
}

#[pymethods]
impl SyntaxHandle {
    #[getter]
    fn name(&self) -> PyResult<&str> {
        Ok(&self.syntax.name)
    }

    #[getter]
    fn language(&self) -> PyResult<String> {
        let scope_name = self.syntax.scope.build_string();
        Ok(scope_name)
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

    fn find_syntax_by_extension(&self, extension: &str) -> PyResult<SyntaxHandle> {
        let result = self.syntax_set.find_syntax_by_extension(extension).cloned();
        match result {
           Some(syntax) => {
               let result = SyntaxHandle {
                   syntax: syntax
               };
               Ok(result)
           },
           None => Err(SyntaxNotFoundError::py_err(
               format!("{} extension not found", extension)))
       }
    }

}

#[pyclass(name=Theme)]
#[derive(Debug, Clone)]
pub struct ThemeHandle {
    pub theme: Theme,
}

#[pymethods]
impl ThemeHandle {
    #[getter]
    fn name(&self) -> PyResult<&str> {
        let default = "None";
        match &self.theme.name {
            Some(x) => Ok(x.as_str()),
            None => Ok(&default)
        }
    }
}

#[pyclass(name=ThemeSet)]
pub struct ThemeSetHandle {
    pub theme_set: ThemeSet,
}

#[pyproto]
impl PyMappingProtocol for ThemeSetHandle {
    fn __getitem__(&self, key: String) -> PyResult<ThemeHandle> {
        let opt = self.theme_set.themes.get(&key).cloned();
        match opt {
            Some(v) => {
                let handler = ThemeHandle {
                    theme: v
                };
                Ok(handler)
            },
            None => Err(PyErr::new::<exceptions::KeyError, _>(
                format!("Theme {} does not exist", key)
            ))
        }
    }
}

#[pymethods]
impl ThemeSetHandle {
    #[getter]
    fn themes(&self) -> PyResult<Vec<&String>> {
        let mut themes = Vec::<&String>::new();
        for key in self.theme_set.themes.keys() {
            themes.push(key)
        }
        Ok(themes)
    }

    #[staticmethod]
    fn load_defaults() -> PyResult<ThemeSetHandle> {
        let defaults = ThemeSet::load_defaults();
        let handler = ThemeSetHandle {
            theme_set: defaults
        };
        Ok(handler)
    }
}

// Exception classes
create_exception!(pysyntect, LoadingError, exceptions::Exception);
create_exception!(pysyntect, SyntaxNotFoundError, exceptions::Exception);

// Module methods
#[pyfunction]
/// Test function for syntect execution
fn highlight(
    text: &'static str,
    language: &str,
    syntax_set: SyntaxSetHandle,
    theme: ThemeHandle
) -> PyResult<Vec<(StyleWrap, &'static str)>> {
    // let ps = SyntaxSet::load_defaults_newlines();
    let ps = syntax_set.syntax_set;
    // let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(language).unwrap();
    // for key in ts.themes.keys() {
    //     println!("{}", key);
    // }
    let mut h = HighlightLines::new(syntax, &theme.theme);
    // let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (light)"]);
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
                style: style,
            };

            python_output.push((py_style, text));
        }
        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        println!("{}", escaped);
    }
    Ok(python_output)
}

#[pyfunction]
fn load_syntax_folder(folder: &'static str) -> PyResult<SyntaxSetHandle> {
    let syn_set = SyntaxSet::load_from_folder(folder);
    match syn_set {
        Ok(result) => {
            // let syntaxes = result.syntaxes();
            let syn_handle = SyntaxSetHandle { syntax_set: result };
            Ok(syn_handle)
        }
        Err(err) => Err(LoadingError::py_err(err.to_string())),
    }
}

#[pyfunction]
fn load_theme_folder(folder: &'static str) -> PyResult<ThemeSetHandle> {
    let theme_set = ThemeSet::load_from_folder(folder);
    match theme_set {
        Ok(result) => {
            let theme_handle = ThemeSetHandle { theme_set: result };
            Ok(theme_handle)
        }
        Err(err) => Err(LoadingError::py_err(err.to_string())),
    }
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn syntect(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_wrapped(wrap_pyfunction!(highlight))?;
    m.add_wrapped(wrap_pyfunction!(load_syntax_folder))?;
    m.add_wrapped(wrap_pyfunction!(load_theme_folder))?;
    m.add_class::<StyleWrap>()?;
    m.add_class::<ColorWrap>()?;
    m.add_class::<FontStyleWrap>()?;
    m.add_class::<SyntaxHandle>()?;
    m.add_class::<SyntaxSetHandle>()?;
    m.add_class::<ThemeHandle>()?;
    m.add_class::<ThemeSetHandle>()?;
    Ok(())
}
