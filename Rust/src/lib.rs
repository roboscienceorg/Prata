#![allow(non_snake_case)]

extern crate pyo3;
extern crate zmq;

mod channel;
mod master;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::str;
use std::ffi::CStr;
use std::os::raw::c_char;
use master::*;
use channel::*;


#[pymodule]
fn TALA(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(connect, m)?)?;
    m.add_function(wrap_pyfunction!(disconnect, m)?)?; m.add_function(wrap_pyfunction!(subscriber, m)?)?; m.add_function(wrap_pyfunction!(publisher, m)?)?; m.add_function(wrap_pyfunction!(gui, m)?)?;

    m.add_class::<Master>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Publisher>()?;

    Ok(())
}

#[pyfunction]
fn connect(py: Python, IP: String, p: u16) -> Py<Master>{
    Py::new(py, Master{ipAddress: IP, port: p}).unwrap()
}

#[pyfunction]
fn disconnect(_py: Python, ms: Master) -> PyResult<()>{
    ms.disconnect();
    Ok(())
}

#[pyfunction]
fn subscriber(py: Python, chn: String, ms: Master) -> Py<Subscriber>{
    Py::new(py, ms.subscriber(chn)).unwrap()
}

#[pyfunction]
fn publisher(py: Python, chn: String, ms: Master) -> Py<Publisher>{
    Py::new(py, ms.publisher(chn)).unwrap()
}

#[pyfunction]
fn gui(py: Python, ms: Master) -> PyResult<()>{
    ms.gui();
    Ok(())
}

