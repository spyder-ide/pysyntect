/// Public functions to escape highlighted text
// PyO3 imports
use pyo3::prelude::*;

// Syntect imports
use syntect::highlighting::Style;
use syntect::util::{as_24_bit_terminal_escaped, as_latex_escaped};

// Local imports
use crate::base::style::StyleWrap;

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
pub fn escape_to_console(all_ranges: Vec<(StyleWrap, &str)>, display_bg: bool) -> PyResult<()> {
    let transformed_ranges: Vec<(Style, &str)> = all_ranges
        .iter()
        .map(|(style_wrap, text)| (style_wrap.style, *text))
        .collect();
    let escaped = as_24_bit_terminal_escaped(&transformed_ranges[..], display_bg);
    println!("{}", escaped);
    println!("\x1b[0m");
    Ok(())
}

/// Output highlighted text as LaTeX, given a list of pairs (token, style).
///
/// Parameters
/// ------
/// all_ranges: List[Tuple[Style, str]]
///     List of tuples (style, token), where each style describes the colors
///     that should be applied to the corresponding token.
#[pyfunction]
pub fn escape_to_latex(all_ranges: Vec<(StyleWrap, &str)>) -> PyResult<String> {
    // let output = Vec::<String>::new();
    let transformed_ranges: Vec<(Style, &str)> = all_ranges
        .iter()
        .map(|(style_wrap, text)| (style_wrap.style, *text))
        .collect();
    let escaped = as_latex_escaped(&transformed_ranges[..]);
    Ok(escaped)
}
