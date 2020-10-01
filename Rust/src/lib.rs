#![allow(non_snake_case)]

extern crate pyo3;
extern crate zmq;
extern crate serde_json;
extern crate serde;
extern crate serde_derive;

mod channel;
mod master;
mod publisher;
mod subscriber;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

// use std::str;
// use std::ffi::CStr;
// use std::os::raw::c_char;

use splay::SplayMap;
use splay::SplaySet;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use serde_json::Value as JsonValue;
use std::collections::HashMap;


use master::*;
use channel::*;


#[pymodule]
fn TALA(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(connect, m)?)?;
    m.add_function(wrap_pyfunction!(disconnect, m)?)?;
    m.add_function(wrap_pyfunction!(host, m)?)?;
    m.add_function(wrap_pyfunction!(subscriber, m)?)?;
    m.add_function(wrap_pyfunction!(publisher, m)?)?;
    m.add_function(wrap_pyfunction!(pubLeave, m)?)?;
    m.add_function(wrap_pyfunction!(subLeave, m)?)?;
    m.add_function(wrap_pyfunction!(pubJoin, m)?)?;
    m.add_function(wrap_pyfunction!(subJoin, m)?)?;

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
fn host(_py: Python, ms: Master) -> PyResult<()>{
    ms.host();
    Ok(())
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
fn pubLeave(_py: Python, pb: Publisher, chan: String) -> PyResult<()>{
    pb.disconnect(chan);
}

#[pyfunction]
fn subLeave(_py: Python, sb: Subscriber, chan: String) -> PyResult<()>{
    sb.disconnect(chan);
}

#[pyfunction]
fn pubJoin(_py: Python, pb: Publisher, chan: String) -> PyResult<()>{
    pb.connect(chan);
}

#[pyfunction]
fn subJoin(_py: Python, sb: Subscriber, chan: String) -> PyResult<()>{
    sb.connect(chan);
}

