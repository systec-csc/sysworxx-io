// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use pyo3::exceptions::{PyNotImplementedError, PyOSError, PyRuntimeError, PyValueError};
use pyo3::PyResult;

pub(crate) trait ResultExt<T> {
    fn to_py_result(self) -> PyResult<T>;
}

impl<T> ResultExt<T> for Result<T, sysworxx_io::error::Error> {
    fn to_py_result(self) -> PyResult<T> {
        use sysworxx_io::error::Error as E;
        let err = match self {
            Ok(value) => return Ok(value),
            Err(e) => e,
        };

        let msg = err.to_string();

        match &err {
            E::InvalidChannel => Err(PyValueError::new_err(msg)),
            E::InvalidParameter => Err(PyValueError::new_err(msg)),
            E::NotImplemented => Err(PyNotImplementedError::new_err(msg)),
            E::WatchdogTimeout => Err(PyRuntimeError::new_err(msg)),
            E::AccessFailed(code) => Err(PyOSError::new_err(format!("{err} ({code})"))),
            E::ParseIntError => Err(PyRuntimeError::new_err(msg)),
            E::GenericError => Err(PyRuntimeError::new_err(msg)),
        }
    }
}
