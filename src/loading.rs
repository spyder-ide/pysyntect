/// Public functions to load syntax definitions and highlighting themes

// PyO3 imports
use pyo3::prelude::*;

// Syntect imports
use syntect::parsing::SyntaxSet;
use syntect::highlighting::ThemeSet;

// Local imports
use crate::base::syntax::SyntaxSetHandle;
use crate::base::theme::ThemeSetHandle;
use crate::exceptions::LoadingError;


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
pub fn load_syntax_folder(folder: String) -> PyResult<SyntaxSetHandle> {
    let syn_set = SyntaxSet::load_from_folder(&folder);
    match syn_set {
        Ok(result) => {
            // let syntaxes = result.syntaxes();
            let syn_handle = SyntaxSetHandle { syntax_set: result };
            Ok(syn_handle)
        }
        Err(err) => Err(LoadingError::new_err(err.to_string())),
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
pub fn load_theme_folder(folder: String) -> PyResult<ThemeSetHandle> {
    let theme_set = ThemeSet::load_from_folder(&folder);
    match theme_set {
        Ok(result) => {
            let theme_handle = ThemeSetHandle { theme_set: result };
            Ok(theme_handle)
        }
        Err(err) => Err(LoadingError::new_err(err.to_string())),
    }
}
