// SPDX-License-Identifier: LGPL-3.0-or-later
// SPDX-FileCopyrightText: 2025 SYS TEC electronic AG <https://www.systec-electronic.com/>

use std::cell::{RefCell, RefMut};
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::sync;

use indexmap::IndexMap;
use once_cell::sync::OnceCell;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::sync::GILProtected;
use pyo3::types::PyFunction;
use pyo3_stub_gen::{define_stub_info_gatherer, derive::*};
use serde::Serialize;

use sysworxx_io::ffi::IoBool;
use sysworxx_io::{Io, IoChannel};

mod result;
mod types;

use result::ResultExt;
use types::*;

static INSTANCE: OnceCell<sync::Mutex<Io>> = OnceCell::new();

/// Get or init `Io` singleton instance
fn instance() -> PyResult<sync::MutexGuard<'static, Io>> {
    let instance: &sync::Mutex<Io> = INSTANCE
        .get_or_try_init(|| {
            let mut io = Io::new()?;
            io.init()?;
            Ok(sync::Mutex::new(io))
        })
        .to_py_result()?;

    match instance.lock() {
        Ok(io) => Ok(io),
        Err(err) => Err(PyRuntimeError::new_err(format!("{}", err))),
    }
}

struct PyFuncHashable {
    func: Py<PyFunction>,
}

impl PartialEq for PyFuncHashable {
    fn eq(&self, other: &Self) -> bool {
        self.func.as_ptr() == other.func.as_ptr()
    }
}

impl Eq for PyFuncHashable {}

impl Hash for PyFuncHashable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.func.as_ptr().hash(state);
    }
}

impl From<Py<PyFunction>> for PyFuncHashable {
    fn from(func: Py<PyFunction>) -> Self {
        PyFuncHashable { func }
    }
}

static CALLBACKS: OnceCell<GILProtected<RefCell<HashMap<usize, HashSet<PyFuncHashable>>>>> =
    OnceCell::new();

/// Singleton which holds all registered callbacks of all input channels.
fn callbacks(py: Python<'_>) -> RefMut<'_, HashMap<usize, HashSet<PyFuncHashable>>> {
    CALLBACKS
        .get_or_init(|| GILProtected::new(RefCell::new(HashMap::new())))
        .get(py)
        .borrow_mut()
}

#[gen_stub_pyclass]
#[pyclass(frozen, get_all)]
#[derive(Serialize, Debug)]
/// Provides all available channels and their respective names.
struct ChannelInfo {
    run_led: bool,
    err_led: bool,
    run_switch: bool,
    config_switch: bool,
    outputs: IndexMap<usize, String>,
    inputs: IndexMap<usize, String>,
    analog_inputs: IndexMap<usize, String>,
    analog_outputs: IndexMap<usize, String>,
    temp_sensors: IndexMap<usize, String>,
    counter_inputs: IndexMap<usize, String>,
    pwm_outputs: IndexMap<usize, String>,
}

#[gen_stub_pymethods]
#[pymethods]
impl ChannelInfo {
    fn __str__(&self, py: Python<'_>) -> String {
        format!(
            "{:?}",
            pythonize::pythonize(py, self).expect("pythonize channel info")
        )
    }

    fn __repr__(&self, py: Python<'_>) -> String {
        self.__str__(py)
    }
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get Python object describing all available channels of the system.
fn get_channel_info() -> PyResult<ChannelInfo> {
    let io = instance()?;
    let info = io.get_channel_info();

    fn to_map<T: IoChannel + ?Sized>(
        fallback_label_prefix: &'static str,
        channels: &[Box<T>],
    ) -> IndexMap<usize, String> {
        channels
            .iter()
            .enumerate()
            .filter(|(_, c)| !c.is_dummy())
            .map(|(i, c)| {
                let label = c
                    .label()
                    .map(ToString::to_string)
                    .unwrap_or_else(|| format!("{fallback_label_prefix}{i}"));
                (i, label)
            })
            .collect()
    }

    let info = ChannelInfo {
        run_led: !info.run_led.is_dummy(),
        err_led: !info.err_led.is_dummy(),
        run_switch: !info.run_switch.is_dummy(),
        config_switch: !info.config_switch.is_dummy(),
        outputs: to_map("DO", info.outputs),
        inputs: to_map("DI", info.inputs),
        analog_inputs: to_map("AI", info.analog_inputs),
        analog_outputs: to_map("AO", info.analog_outputs),
        temp_sensors: to_map("TMP", info.temp_sensors),
        counter_inputs: to_map("CNT", info.counter_inputs),
        pwm_outputs: to_map("PWM", info.pwm_outputs),
    };

    Ok(info)
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set state of the `RUN_LED` if available
fn set_run_led(value: bool) -> PyResult<()> {
    instance()?.set_run_led(value).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set state of the `ERROR_LED` if available
fn set_err_led(value: bool) -> PyResult<()> {
    instance()?.set_err_led(value).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get state of the `RUN_SWITCH` if available
fn get_run_switch() -> PyResult<bool> {
    instance()?.get_run_switch().to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get state of the `CONFIG_SWITCH` if available
fn get_config_switch() -> PyResult<bool> {
    instance()?.get_config_switch().to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set state of a digital output
fn output_set(channel: usize, value: bool) -> PyResult<()> {
    instance()?.output_set(channel, value).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get state of a digital input
fn input_get(channel: usize) -> PyResult<bool> {
    instance()?.input_get(channel).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Register a function which will be called if the input signal changes. The callback must have
/// the signature `callback(channel: usize, value: bool)`.
///
/// Registering the same callback for the same channel twice does not have any effect. Registering
/// multiple callback functions for the same channel is possible.
fn input_register_callback(channel: usize, callback: Bound<PyAny>, py: Python) -> PyResult<()> {
    if !callback.is_callable() {
        return Err(PyValueError::new_err("Callback must be callable"));
    }

    let callback: Py<PyFunction> = callback.extract()?;

    let mut callbacks = callbacks(py);

    if !callbacks.contains_key(&channel) {
        instance()?
            .input_register_callback(
                channel,
                Some(input_callback),
                sysworxx_io::ffi::IoInputTrigger::BothEdge,
            )
            .to_py_result()?
    }

    let callbacks_of_channel = callbacks.entry(channel).or_default();
    callbacks_of_channel.insert(callback.into());

    Ok(())
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Un-register a callback of an input channel. (counterpart of `input_register_callback`)
fn input_unregister_callback(channel: usize, callback: Bound<PyAny>, py: Python) -> PyResult<()> {
    let mut callbacks = callbacks(py);
    let callback: Py<PyFunction> = callback.extract()?;

    let callbacks_of_channel = callbacks
        .get_mut(&channel)
        .ok_or(PyValueError::new_err("Channel not registered"))?;

    let removed = callbacks_of_channel.remove(&callback.into());
    if !removed {
        return Err(PyValueError::new_err(
            "Provided callback was not registered",
        ));
    }

    if callbacks_of_channel.is_empty() {
        instance()?
            .input_unregister_callback(channel)
            .to_py_result()?;
        callbacks.remove(&channel);
    }

    Ok(())
}

/// The actual C-level input callback function which will delegate calls to registered input
/// callback functions. (see `input_register_callback`)
extern "C" fn input_callback(channel: u8, value: IoBool) -> () {
    Python::with_gil(|py| {
        for cb in callbacks(py)
            .get(&(channel as usize))
            .expect("input not registered")
        {
            let value: bool = *value;
            let res = cb.func.call(py, (channel, value), None);
            if let Err(err) = res {
                eprintln!("{err}");
            }
        }
    });
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get value of an analog input (ADC)
fn analog_input_get(channel: usize) -> PyResult<i64> {
    instance()?.analog_input_get(channel).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set the mode of an analog input
fn analog_mode_set(channel: usize, mode: AnalogMode) -> PyResult<()> {
    instance()?
        .analog_mode_set(channel, mode.into())
        .to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set the value of an analog output (DAC)
fn analog_output_set(channel: usize, value: i64) -> PyResult<()> {
    instance()?.analog_output_set(channel, value).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set the mode of a temperature sensors. Not all sensors provide all modes and sensor types.
fn tmp_set_mode(channel: usize, mode: TmpMode, sensor_type: TmpSensorType) -> PyResult<()> {
    instance()?
        .tmp_set_mode(channel, mode.into(), sensor_type.into())
        .to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get the value of an temperature sensor.
fn tmp_input_get(channel: usize) -> PyResult<f64> {
    instance()?.tmp_input_get(channel).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Enable the counter or A/B-encoder functionality of a counter.
fn cnt_enable(channel: usize, state: bool) -> PyResult<()> {
    instance()?.cnt_enable(channel, state).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Setup the mode of operation of a counter input
fn cnt_setup(
    channel: usize,
    mode: CntMode,
    trigger: CntTrigger,
    direction: CntDirection,
) -> PyResult<()> {
    instance()?
        .cnt_setup(channel, mode.into(), trigger.into(), direction.into())
        .to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set a start value which will be applied when the counter input gets enabled.
fn cnt_set_preload(channel: usize, preload: i32) -> PyResult<()> {
    instance()?.cnt_set_preload(channel, preload).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Get the current counter value
fn cnt_get(channel: usize) -> PyResult<i32> {
    instance()?.cnt_get(channel).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Enable a PWM output
fn pwm_enable(channel: usize, state: bool) -> PyResult<()> {
    instance()?.pwm_enable(channel, state).to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Setup the PWM period and duty cycle of a PWM channel.
fn pwm_setup(channel: usize, period: u16, duty_cycle: u16) -> PyResult<()> {
    instance()?
        .pwm_setup(channel, period, duty_cycle)
        .to_py_result()
}

#[gen_stub_pyfunction]
#[pyfunction]
/// Set the time base of a PWM channel. Not all PWM channels support all time bases.
fn pwm_set_timebase(channel: usize, timebase: PwmTimebase) -> PyResult<()> {
    instance()?
        .pwm_set_timebase(channel, timebase.into())
        .to_py_result()
}

#[pymodule]
/// Python bindings to `sysworxx-io` library.
fn sysworxx_io_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ChannelInfo>()?;

    m.add_class::<InputTrigger>()?;
    m.add_class::<AnalogMode>()?;
    m.add_class::<TmpMode>()?;
    m.add_class::<TmpSensorType>()?;
    m.add_class::<CntMode>()?;
    m.add_class::<CntTrigger>()?;
    m.add_class::<CntDirection>()?;
    m.add_class::<PwmTimebase>()?;

    // m.add_function(wrap_pyfunction!(get_ticks, m)?)?;
    // m.add_function(wrap_pyfunction!(watchdog_enable, m)?)?;
    // m.add_function(wrap_pyfunction!(watchdog_service, m)?)?;
    m.add_function(wrap_pyfunction!(get_channel_info, m)?)?;
    // m.add_function(wrap_pyfunction!(write_json_info, m)?)?;
    m.add_function(wrap_pyfunction!(set_run_led, m)?)?;
    m.add_function(wrap_pyfunction!(set_err_led, m)?)?;
    m.add_function(wrap_pyfunction!(get_run_switch, m)?)?;
    m.add_function(wrap_pyfunction!(get_config_switch, m)?)?;
    m.add_function(wrap_pyfunction!(output_set, m)?)?;
    m.add_function(wrap_pyfunction!(input_get, m)?)?;
    m.add_function(wrap_pyfunction!(input_register_callback, m)?)?;
    m.add_function(wrap_pyfunction!(input_unregister_callback, m)?)?;
    m.add_function(wrap_pyfunction!(analog_input_get, m)?)?;
    m.add_function(wrap_pyfunction!(analog_mode_set, m)?)?;
    m.add_function(wrap_pyfunction!(analog_output_set, m)?)?;
    m.add_function(wrap_pyfunction!(tmp_set_mode, m)?)?;
    m.add_function(wrap_pyfunction!(tmp_input_get, m)?)?;
    m.add_function(wrap_pyfunction!(cnt_enable, m)?)?;
    m.add_function(wrap_pyfunction!(cnt_setup, m)?)?;
    m.add_function(wrap_pyfunction!(cnt_set_preload, m)?)?;
    m.add_function(wrap_pyfunction!(cnt_get, m)?)?;
    m.add_function(wrap_pyfunction!(pwm_enable, m)?)?;
    m.add_function(wrap_pyfunction!(pwm_setup, m)?)?;
    m.add_function(wrap_pyfunction!(pwm_set_timebase, m)?)?;
    Ok(())
}

define_stub_info_gatherer!(stub_info);
