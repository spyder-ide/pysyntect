/// Class wrapper around [`syntect::parsing::SyntaxSet`] and [`syntect::parsing::SyntaxReference`]
// PyO3 imports
use pyo3::prelude::*;

// Syntect imports
use syntect::parsing::{SyntaxReference, SyntaxSet};

// Local imports
use crate::exceptions::SyntaxNotFoundError;

/// Handle to a language syntax definition.
#[pyclass(name = "Syntax")]
pub struct SyntaxHandle {
    pub syntax: SyntaxReference,
}

#[pymethods]
impl SyntaxHandle {
    /// Name of the syntax definition provider.
    #[getter]
    fn name(&self) -> &str {
        &self.syntax.name
    }

    /// Name of the language supported by the syntax.
    #[getter]
    fn language(&self) -> String {
        self.syntax.scope.build_string()
    }
}

/// Handle to a list of language syntax definitions.
#[pyclass(name = "SyntaxSet")]
#[derive(Debug, Clone)]
pub struct SyntaxSetHandle {
    pub syntax_set: SyntaxSet,
}

#[pymethods]
impl SyntaxSetHandle {
    /// List of languages included on this syntax list.
    #[getter]
    fn languages(&self) -> Vec<&str> {
        let mut languages = Vec::<&str>::new();
        for syntax in self.syntax_set.syntaxes() {
            languages.push(&syntax.name);
        }
        languages
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
                let result = SyntaxHandle { syntax };
                Ok(result)
            }
            None => Err(SyntaxNotFoundError::new_err(format!(
                "{} extension not found",
                extension
            ))),
        }
    }
}
