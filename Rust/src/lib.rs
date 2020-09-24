#![allow(non_snake_case)]

extern crate pyo3;
extern crate zmq;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::str;
use std::ffi::CStr;
use std::os::raw::c_char;

//place holder
#[pyclass]
struct Master{}

#[pyclass]
struct Subscriber{}

#[pymethods]
impl Subscriber{
  fn connect(&self, channel: String){
    println!("Subscriber Connected");
  }
}

#[pymethods]
impl Master{
  fn host(&self){
    println!("Hosted");
  }

}

#[pymodule]
fn TALA(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_wrapped(wrap_pyfunction!(host))?;
    m.add_wrapped(wrap_pyfunction!(subscriber))?;
    m.add_wrapped(wrap_pyfunction!(installTest))?;

    m.add_class::<Master>()?;
    m.add_class::<Subscriber>()?;

    Ok(())

}

//Python connectors

#[pyfunction]
fn installTest(_py: Python){
  println!("The Test succeeded you can now use TALA");
}

//Python Host Master
#[pyfunction]
fn host(py: Python) -> Py<Master> {
  Py::new(py, Master{}).unwrap()
}

//Python Get subscriber
#[pyfunction]
fn subscriber(py: Python) -> Py<Subscriber> {
  Py::new(py, Subscriber{}).unwrap()
  //subscriber(channelName.as_str());
  //Ok(())
}
