/// Class wrappers around syntect structs

// Local modules
pub mod color;
pub mod style;
pub mod syntax;
pub mod theme;

// PyO3 imports
use pyo3::prelude::*;

// Local imports
pub use self::color::ColorWrap;
pub use self::style::StyleWrap;
pub use self::syntax::{SyntaxHandle, SyntaxSetHandle};
pub use self::theme::{ThemeHandle, ThemeSetHandle};


/// Enum class that contains font style constants
#[pyclass(name="FontStyleConst")]
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
    fn bold() -> i8 {
        FontStyleWrap::BOLD
    }

    #[staticmethod]
    fn underline() -> i8 {
        FontStyleWrap::UNDERLINE
    }

    #[staticmethod]
    fn italic() -> i8 {
        FontStyleWrap::ITALIC
    }
}