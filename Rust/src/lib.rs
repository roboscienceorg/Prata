#![allow(non_snake_case)]
#[allow(dead_code)]
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
//use std::net::Ipv4Addr;
//use port_scanner::request_open_port;

use master::Master;
use publisher::Publisher;
use subscriber::Subscriber;



#[pymodule]
fn TALA(_py: Python, m: &PyModule) -> PyResult<()> {

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    #[pyfn(m, "connect")]
    fn connect(py: Python, IP: String, p: u16) -> Py<Master>{

        // let p = request_open_port().unwrap_or(0);
        // let addr = (Ipv4Addr::LOCALHOST).to_string();
        Py::new(py, Master{ipAddress: IP, port: p, threading: true}).unwrap()
    }

    m.add_class::<Master>()?;
    m.add_class::<Subscriber>()?;
    m.add_class::<Publisher>()?;

    Ok(())
}
