/// Exception definitions

// PyO3 imports
use pyo3::exceptions;
use pyo3::create_exception;

create_exception!(pysyntect, LoadingError, exceptions::PyException);
create_exception!(pysyntect, SyntaxNotFoundError, exceptions::PyException);
