// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use pyo3::prelude::*;
use pyo3_stub_gen::derive::gen_stub_pyclass_enum;

use sysworxx_io::ffi::{
    IoAnalogMode, IoCntDirection, IoCntMode, IoCntTrigger, IoInputTrigger, IoPwmTimebase,
    IoTmpMode, IoTmpSensorType,
};

/// Simple conversion from one enum type to another (when variants are identical)
macro_rules! enum_convert {
    ($from:ident, $to:ident, {$($variant:ident),* $(,)?}) => {
        #[gen_stub_pyclass_enum]
        #[pyclass(rename_all = "UPPERCASE")]
        #[derive(Clone)]
        pub enum $from {
            $($variant),*
        }

        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                match value {
                    $($from::$variant => $to::$variant),*
                }
            }
        }
    };
}

enum_convert!(InputTrigger, IoInputTrigger, {
    None,
    RisingEdge,
    FallingEdge,
    BothEdge,
});

enum_convert!(AnalogMode, IoAnalogMode, {
    Current,
    Voltage
});

enum_convert!(TmpMode, IoTmpMode, {
    RtdTwoWire,
    RtdThreeWire,
    RtdFourWire,
});

enum_convert!(TmpSensorType, IoTmpSensorType, {
    PT100,
    PT1000,
});

enum_convert!(CntMode, IoCntMode, {
    Counter,
    ABEncoder,
});

enum_convert!(CntTrigger, IoCntTrigger, {
    RisingEdge,
    FallingEdge,
    AnyEdge,
});

enum_convert!(CntDirection, IoCntDirection, {
    Up,
    Down,
});

enum_convert!(PwmTimebase, IoPwmTimebase, {
    Ns800,
    Ms1,
});
