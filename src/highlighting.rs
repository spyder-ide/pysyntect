/// Highlight functions

// PyO3 imports
use pyo3::prelude::*;

// Syntect imports
use syntect::easy::HighlightLines;
use syntect::util::LinesWithEndings;
use syntect::highlighting::Style;

// Local imports
use crate::base::color::ColorWrap;
use crate::base::style::StyleWrap;
use crate::base::theme::ThemeHandle;
use crate::base::syntax::SyntaxSetHandle;

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
pub fn highlight(
    text: String,
    language: String,
    syntax_set: SyntaxSetHandle,
    theme: ThemeHandle
) -> PyResult<Vec<(StyleWrap, String)>> {
    // let ps = SyntaxSet::load_defaults_newlines();
    let ps = syntax_set.syntax_set;
    // let ts = ThemeSet::load_defaults();

    let syntax = ps.find_syntax_by_extension(&language).unwrap();
    // for key in ts.themes.keys() {
    //     println!("{}", key);
    // }
    let mut h = HighlightLines::new(syntax, &theme.theme);
    // let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (light)"]);
    // let s = "def tefa(a, b, *args, **kwargs):\n    return a + b";
    let s_slice: &str = &*text;
    let mut python_output = Vec::<(StyleWrap, String)>::new();
    for line in LinesWithEndings::from(s_slice) {
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

            python_output.push((py_style, text.to_string()));
        }
        // let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
        // println!("{}", escaped);
    }
    Ok(python_output)
}
