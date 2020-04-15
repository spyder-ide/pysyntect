// ----------------------------------------------------------------------------
// Copyright (c) Spyder Project Contributors
//
// Licensed under the terms of the MIT License
// (see LICENSE.txt for details)
// ----------------------------------------------------------------------------

/// Rust bindings to syntect library in Python

// PyO3 imports
use pyo3::create_exception;
use pyo3::exceptions;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::class::PyMappingProtocol;

// Syntect imports
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, Theme, ThemeSet, Color};
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

// Package version
const VERSION: &'static str = env!("CARGO_PKG_VERSION");

// Class wrappers around syntect structs
// ----------------------------------------------------------------------------

/// RGBA color descriptor
#[pyclass(name=Color)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorWrap {
    /// Handle to original color
    color: Color
}

#[pymethods]
impl ColorWrap {
    /// Red component.
    #[getter]
    fn r(&self) -> PyResult<u8> {
        Ok(self.color.r)
    }

    /// Green component.
    #[getter]
    fn g(&self) -> PyResult<u8> {
        Ok(self.color.g)
    }

    /// Blue component.
    #[getter]
    fn b(&self) -> PyResult<u8> {
        Ok(self.color.b)
    }

    /// Alpha component.
    #[getter]
    fn a(&self) -> PyResult<u8> {
        Ok(self.color.a)
    }
}

/// Style color description used to highlight a token.
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
    /// Foreground color.
    #[getter]
    fn foreground(&self) -> PyResult<ColorWrap> {
        Ok(self.foreground)
    }

    /// Background color.
    #[getter]
    fn background(&self) -> PyResult<ColorWrap> {
        Ok(self.background)
    }

    /// Font style.
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

/// Handle to a language syntax definition.
#[pyclass(name=Syntax)]
pub struct SyntaxHandle {
    pub syntax: SyntaxReference
}

#[pymethods]
impl SyntaxHandle {
    /// Name of the syntax definition provider.
    #[getter]
    fn name(&self) -> PyResult<&str> {
        Ok(&self.syntax.name)
    }

    /// Name of the language supported by the syntax.
    #[getter]
    fn language(&self) -> PyResult<String> {
        let scope_name = self.syntax.scope.build_string();
        Ok(scope_name)
    }

}


/// Handle to a list of language syntax definitions.
#[pyclass(name=SyntaxSet)]
#[derive(Debug, Clone)]
pub struct SyntaxSetHandle {
    pub syntax_set: SyntaxSet,
}

#[pymethods]
impl SyntaxSetHandle {
    /// List of languages included on this syntax list.
    #[getter]
    fn languages(&self) -> PyResult<Vec<&str>> {
        let mut languages = Vec::<&str>::new();
        for syntax in self.syntax_set.syntaxes() {
            languages.push(&syntax.name);
        }
        Ok(languages)
    }

    /// Find language syntax descriptor.
    ///
    /// Find and return the syntax definition for a language, given
    /// its file extension.
    ///
    /// Parameters
    /// ----------
    /// extension: str
    ///     Extension name of the language to lookup.
    ///
    /// Returns
    /// -------
    /// syntax: Syntax
    ///     Handle to the language syntax definition.
    ///
    /// Raises
    /// ------
    /// SyntaxNotFoundError
    ///     If the provided language file extension does not have a known
    ///     syntax definition on this set.
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

fn pack_color(color_opt: Option<Color>) -> Option<ColorWrap> {
    match color_opt {
        Some(color) => {
            let wrapped_color = ColorWrap {
                color: color
            };
            Some(wrapped_color)
        }
        None => None
    }
}

/// Handle to a theme descriptor.
///
/// This class points and refers to the main colors and settings defined by
/// the theme.
#[pyclass(name=Theme)]
#[derive(Debug, Clone)]
pub struct ThemeHandle {
    pub theme: Theme,
}

#[pymethods]
impl ThemeHandle {
    /// Name of the theme.
    #[getter]
    fn name(&self) -> PyResult<&str> {
        let default = "None";
        match &self.theme.name {
            Some(x) => Ok(x.as_str()),
            None => Ok(&default)
        }
    }

    /// The default backgound color of the view.
    #[getter]
    fn background(&self) -> PyResult<Option<ColorWrap>> {
        let background_color = self.theme.settings.background;
        Ok(pack_color(background_color))
    }

    /// The default color for text.
    #[getter]
    fn foreground(&self) -> PyResult<Option<ColorWrap>> {
        let foreground_color = self.theme.settings.foreground;
        Ok(pack_color(foreground_color))
    }

    /// Color of the caret.
    #[getter]
    fn caret(&self) -> PyResult<Option<ColorWrap>> {
        let caret_color = self.theme.settings.caret;
        Ok(pack_color(caret_color))
    }

    /// Color of the line the caret is in.
    #[getter]
    fn line_highlight(&self) -> PyResult<Option<ColorWrap>> {
        let line_highlight_color = self.theme.settings.line_highlight;
        Ok(pack_color(line_highlight_color))
    }

    /// The color to use for the squiggly underline drawn under misspelled words.
    #[getter]
    fn misspelling(&self) -> PyResult<Option<ColorWrap>> {
        let misspelling_color = self.theme.settings.misspelling;
        Ok(pack_color(misspelling_color))
    }

    /// A color made available for use by the theme.
    #[getter]
    fn accent(&self) -> PyResult<Option<ColorWrap>> {
        let accent_color = self.theme.settings.accent;
        Ok(pack_color(accent_color))
    }

    /// Color of bracketed sections of text when the caret is in a bracketed section.
    #[getter]
    fn bracket_contents_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let bracket_contents_foreground_color = self.theme.settings.bracket_contents_foreground;
        Ok(pack_color(bracket_contents_foreground_color))
    }

    /// Foreground color of the brackets when the caret is next to a bracket.
    #[getter]
    fn brackets_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let brackets_foreground_color = self.theme.settings.brackets_foreground;
        Ok(pack_color(brackets_foreground_color))
    }

    /// Background color of the brackets when the caret is next to a bracket.
    #[getter]
    fn brackets_background(&self) -> PyResult<Option<ColorWrap>> {
        let brackets_background_color = self.theme.settings.brackets_background;
        Ok(pack_color(brackets_background_color))
    }

    /// Color of tags when the caret is next to a tag.
    #[getter]
    fn tags_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let tags_foreground_color = self.theme.settings.tags_foreground;
        Ok(pack_color(tags_foreground_color))
    }

    /// The border color for "other" matches.
    #[getter]
    fn highlight(&self) -> PyResult<Option<ColorWrap>> {
        let highlight_color = self.theme.settings.highlight;
        Ok(pack_color(highlight_color))
    }

    /// Background color of regions matching the current search.
    #[getter]
    fn find_highlight(&self) -> PyResult<Option<ColorWrap>> {
        let find_highlight_color = self.theme.settings.find_highlight;
        Ok(pack_color(find_highlight_color))
    }

    /// Text color of regions matching the current search.
    #[getter]
    fn find_highlight_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let find_highlight_foreground_color = self.theme.settings.find_highlight_foreground;
        Ok(pack_color(find_highlight_foreground_color))
    }

    /// Background color of the gutter.
    #[getter]
    fn gutter(&self) -> PyResult<Option<ColorWrap>> {
        let gutter_color = self.theme.settings.gutter;
        Ok(pack_color(gutter_color))
    }

    /// Foreground color of the gutter.
    #[getter]
    fn gutter_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let gutter_foreground_color = self.theme.settings.gutter_foreground;
        Ok(pack_color(gutter_foreground_color))
    }

    /// The background color of selected text.
    #[getter]
    fn selection(&self) -> PyResult<Option<ColorWrap>> {
        let selection_color = self.theme.settings.selection;
        Ok(pack_color(selection_color))
    }

    /// A color that will override the scope-based text color of the selection.
    #[getter]
    fn selection_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let selection_foreground_color = self.theme.settings.selection_foreground;
        Ok(pack_color(selection_foreground_color))
    }

    /// Color of the selection regions border.
    #[getter]
    fn selection_border(&self) -> PyResult<Option<ColorWrap>> {
        let selection_border_color = self.theme.settings.selection_border;
        Ok(pack_color(selection_border_color))
    }

    /// The background color of a selection in a view that is not currently focused.
    #[getter]
    fn inactive_selection(&self) -> PyResult<Option<ColorWrap>> {
        let inactive_selection_color = self.theme.settings.inactive_selection;
        Ok(pack_color(inactive_selection_color))
    }

    /// A color that will override the scope-based text color of the selection
    /// in a view that is not currently focused.
    #[getter]
    fn inactive_selection_foreground(&self) -> PyResult<Option<ColorWrap>> {
        let inactive_selection_foreground_color = self.theme.settings.inactive_selection_foreground;
        Ok(pack_color(inactive_selection_foreground_color))
    }

    /// Color of the guides displayed to indicate nesting levels.
    #[getter]
    fn guide(&self) -> PyResult<Option<ColorWrap>> {
        let guide_color = self.theme.settings.guide;
        Ok(pack_color(guide_color))
    }

    /// Color of the guide lined up with the caret.
    #[getter]
    fn active_guide(&self) -> PyResult<Option<ColorWrap>> {
        let active_guide_color = self.theme.settings.active_guide;
        Ok(pack_color(active_guide_color))
    }

    /// Color of the current guide’s parent guide level.
    #[getter]
    fn stack_guide(&self) -> PyResult<Option<ColorWrap>> {
        let stack_guide_color = self.theme.settings.stack_guide;
        Ok(pack_color(stack_guide_color))
    }

    /// The color of the shadow used when a text area can be horizontally scrolled.
    #[getter]
    fn shadow(&self) -> PyResult<Option<ColorWrap>> {
        let shadow_color = self.theme.settings.shadow;
        Ok(pack_color(shadow_color))
    }

}

/// Handle to a map of highlighting themes.
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
    /// List of theme names included on this map.
    #[getter]
    fn themes(&self) -> PyResult<Vec<&String>> {
        let mut themes = Vec::<&String>::new();
        for key in self.theme_set.themes.keys() {
            themes.push(key)
        }
        Ok(themes)
    }

    /// Load syntect default theme sets.
    ///
    /// Returns
    /// -------
    /// theme_set: ThemeSet
    ///     Handle to a list of syntax highlighting themes.
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

// Module functions
// ----------------------------------------------------------------------------

/// Compute style definitions to highlight a given text of a given language.
///
/// Parameters
/// ------
/// text: str
///     Text to highlight
/// language: str
///     Language extension of the language
/// syntax_set: SyntaxSet
///     List of syntax defintions for multiple languages
/// theme: Theme
///     Handle to the theme to apply
///
/// Returns
/// -------
/// styles: List[Style]
///     List of style transformations to apply to each token in the text.
#[pyfunction]
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
                color: style.foreground
            };
            let py_background = ColorWrap {
                color: style.background
            };

            let py_style = StyleWrap {
                foreground: py_foreground,
                background: py_background,
                font_style: style.font_style.bits(),
                style: *style,
            };

            python_output.push((py_style, text));
        }
        // let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        // println!("{}", escaped);
    }
    Ok(python_output)
}

/// Load syntax defintion files in .sublime-syntax format located on a folder.
///
/// Parameters
/// ------
/// folder: str
///     Path that points to a folder containing syntax definition files.
///
/// Returns
/// -------
/// syntax_set: SyntaxSet
///     Handle to a list of syntax definition objects.
///
/// Raises
/// ------
/// LoadingError:
///     If there was some error trying to read any of the files.
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


/// Load syntax highlighting themes defined in .tmTheme format.
///
/// Parameters
/// ------
/// folder: str
///     Path that points to a folder that contains syntax highlighting themes.
///
/// Returns
/// -------
/// theme_set:
///     Handle to a map of theme handles, mapped by the theme names.
///
/// Raises
/// ------
/// LoadingError:
///     If there was some error trying to read any of the files.
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


/// Output highlighted text to console, given a list of pairs (token, style).
///
/// Parameters
/// ------
/// all_ranges: List[Tuple[Style, str]]
///     List of tuples (style, token), where each style describes the colors
///     that should be applied to the corresponding token.
/// display_bg: bool
///     If set to True, then the background colors are printed into the screen.
///     Else, only the foreground colors will be shown.
#[pyfunction]
fn escape_to_console(all_ranges: Vec<(StyleWrap, &str)>, display_bg: bool) -> PyResult<()> {
    let transformed_ranges: Vec<(Style, &str)> = all_ranges.iter().map(
        |(style_wrap, text)| (style_wrap.style, *text)).collect();
    let escaped = as_24_bit_terminal_escaped(&transformed_ranges[..], display_bg);
    println!("{}", escaped);
    println!("\x1b[0m");
    Ok(())
}

/// This module is a python module implemented in Rust.
#[pymodule]
fn syntect(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", VERSION)?;
    m.add_wrapped(wrap_pyfunction!(highlight))?;
    m.add_wrapped(wrap_pyfunction!(load_syntax_folder))?;
    m.add_wrapped(wrap_pyfunction!(load_theme_folder))?;
    m.add_wrapped(wrap_pyfunction!(escape_to_console))?;
    m.add_class::<StyleWrap>()?;
    m.add_class::<ColorWrap>()?;
    m.add_class::<FontStyleWrap>()?;
    m.add_class::<SyntaxHandle>()?;
    m.add_class::<SyntaxSetHandle>()?;
    m.add_class::<ThemeHandle>()?;
    m.add_class::<ThemeSetHandle>()?;
    Ok(())
}
