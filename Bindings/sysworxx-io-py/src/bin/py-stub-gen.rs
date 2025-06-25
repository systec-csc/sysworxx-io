// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use pyo3_stub_gen::Result;

fn main() -> Result<()> {
    let stub = sysworxx_io_py::stub_info()?;
    stub.generate()?;
    Ok(())
}
